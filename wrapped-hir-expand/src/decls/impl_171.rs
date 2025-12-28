macro_rules! deps {
    () => {
        Name!();
        ProcMacro!();
        CrateProcMacros!();
        ExpandError!();
        CustomProcMacroExpander!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl CrateProcMacros { fn get (& self , idx : u32 , err_span : Span) -> Result < & ProcMacro , ExpandError > { let proc_macros = match & self . 0 { Ok (proc_macros) => proc_macros , Err (_) => { return Err (ExpandError :: other (err_span , "internal error: no proc macros for crate" ,)) ; } } ; proc_macros . get (idx as usize) . ok_or_else (| | { ExpandError :: other (err_span , format ! ("internal error: proc-macro index out of bounds: the length is {} but the index is {}" , proc_macros . len () , idx)) }) } pub fn get_error (& self) -> Option < & ProcMacroLoadingError > { self . 0 . as_ref () . err () } # [doc = " Fetch the [`CustomProcMacroExpander`]s and their corresponding names for the given crate."] pub fn list (& self , def_site_ctx : span :: SyntaxContext ,) -> Option < Box < [(crate :: name :: Name , CustomProcMacroExpander , bool)] > > { match & self . 0 { Ok (proc_macros) => Some (proc_macros . iter () . enumerate () . map (| (idx , it) | { let name = crate :: name :: Name :: new_symbol (it . name . clone () , def_site_ctx) ; (name , CustomProcMacroExpander :: new (idx as u32) , it . disabled) }) . collect () ,) , _ => None , } } }
    };
}

impl_171!()