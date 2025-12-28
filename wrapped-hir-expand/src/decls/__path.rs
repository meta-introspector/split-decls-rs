macro_rules! deps {
    () => {
        PathKind!();
        ModPath!();
        Name!();
    };
}

macro_rules! __path {
    () => {
        deps!();
        # [macro_export] macro_rules ! __path { ($ start : ident $ (:: $ seg : ident) *) => ({ $ crate :: __known_path ! ($ start $ (:: $ seg) *) ; $ crate :: mod_path :: ModPath :: from_segments ($ crate :: mod_path :: PathKind :: Abs , vec ! [$ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ start . clone ()) , $ ($ crate :: name :: Name :: new_symbol_root ($ crate :: intern :: sym ::$ seg . clone ()) ,) *]) }) ; }
    };
}

__path!();