macro_rules! deps {
    () => {
        ExpandDatabase!();
        ProcMacro!();
        MacroCallKind!();
        ExpandResult!();
        MacroDefKind!();
        MacroCallId!();
        Attr!();
    };
}

macro_rules! expand_proc_macro {
    () => {
        deps!();
        fn expand_proc_macro (db : & dyn ExpandDatabase , id : MacroCallId ,) -> ExpandResult < Arc < tt :: TopSubtree > > { let loc = db . lookup_intern_macro_call (id) ; let (macro_arg , undo_info , span) = db . macro_arg_considering_derives (id , & loc . kind) ; let (ast , expander) = match loc . def . kind { MacroDefKind :: ProcMacro (ast , expander , _) => (ast , expander) , _ => unreachable ! () , } ; let attr_arg = match & loc . kind { MacroCallKind :: Attr { attr_args : Some (attr_args) , .. } => Some (& * * attr_args) , _ => None , } ; let ExpandResult { value : mut tt , err } = { let span = db . proc_macro_span (ast) ; expander . expand (db , loc . def . krate , loc . krate , & macro_arg , attr_arg , span_with_def_site_ctxt (db , span , id . into () , loc . def . edition) , span_with_call_site_ctxt (db , span , id . into () , loc . def . edition) , span_with_mixed_site_ctxt (db , span , id . into () , loc . def . edition) ,) } ; if let Err (value) = check_tt_count (& tt) { return value . map (| () | Arc :: new (tt :: TopSubtree :: empty (tt :: DelimSpan :: from_single (span)))) ; } fixup :: reverse_fixups (& mut tt , & undo_info) ; ExpandResult { value : Arc :: new (tt) , err } }
    };
}

expand_proc_macro!()