macro_rules! deps {
    () => {
        PathKind!();
        ModPath!();
        Name!();
    };
}

macro_rules! __tool_path {
    () => {
        deps!();
        # [macro_export] macro_rules ! __tool_path { ($ start : ident $ (:: $ seg : ident) *) => ({ $ crate :: mod_path :: ModPath :: from_segments ($ crate :: mod_path :: PathKind :: Plain , vec ! [$ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym :: rust_analyzer) , $ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ start . clone ()) , $ ($ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ seg . clone ()) ,) *]) }) ; }
    };
}

__tool_path!();