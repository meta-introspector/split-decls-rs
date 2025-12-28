macro_rules! deps {
    () => {
        CowArc!();
        ExpandResult!();
        ProcMacro!();
        MacroCallKind!();
        EagerCallInfo!();
        ExpandDatabase!();
        MacroCallLoc!();
        MacroCallId!();
        MacroDefKind!();
    };
}

macro_rules! macro_expand {
    () => {
        deps!();
        fn macro_expand (db : & dyn ExpandDatabase , macro_call_id : MacroCallId , loc : MacroCallLoc ,) -> ExpandResult < (CowArc < tt :: TopSubtree > , MatchedArmIndex) > { let _p = tracing :: info_span ! ("macro_expand") . entered () ; let (ExpandResult { value : (tt , matched_arm) , err } , span) = match loc . def . kind { MacroDefKind :: ProcMacro (..) => { return db . expand_proc_macro (macro_call_id) . map (CowArc :: Arc) . zip_val (None) ; } _ => { let (macro_arg , undo_info , span) = db . macro_arg_considering_derives (macro_call_id , & loc . kind) ; let arg = & * macro_arg ; let res = match loc . def . kind { MacroDefKind :: Declarative (id) => db . decl_macro_expander (loc . def . krate , id) . expand (db , arg . clone () , macro_call_id , span) , MacroDefKind :: BuiltIn (_ , it) => { it . expand (db , macro_call_id , arg , span) . map_err (Into :: into) . zip_val (None) } MacroDefKind :: BuiltInDerive (_ , it) => { it . expand (db , macro_call_id , arg , span) . map_err (Into :: into) . zip_val (None) } MacroDefKind :: BuiltInEager (_ , it) => { let eager = match & loc . kind { MacroCallKind :: FnLike { eager : None , .. } => { return ExpandResult :: ok (CowArc :: Arc (macro_arg . clone ())) . zip_val (None) ; } MacroCallKind :: FnLike { eager : Some (eager) , .. } => Some (& * * eager) , _ => None , } ; let mut res = it . expand (db , macro_call_id , arg , span) . map_err (Into :: into) ; if let Some (EagerCallInfo { error , .. }) = eager { res . err = error . clone () . or (res . err) ; } res . zip_val (None) } MacroDefKind :: BuiltInAttr (_ , it) => { let mut res = it . expand (db , macro_call_id , arg , span) ; fixup :: reverse_fixups (& mut res . value , & undo_info) ; res . zip_val (None) } MacroDefKind :: ProcMacro (_ , _ , _) => unreachable ! () , } ; (ExpandResult { value : res . value , err : res . err } , span) } } ; if ! loc . def . is_include () { if let Err (value) = check_tt_count (& tt) { return value . map (| () | CowArc :: Owned (tt :: TopSubtree :: empty (tt :: DelimSpan :: from_single (span)))) . zip_val (matched_arm) ; } } ExpandResult { value : (CowArc :: Owned (tt) , matched_arm) , err } }
    };
}

macro_expand!();