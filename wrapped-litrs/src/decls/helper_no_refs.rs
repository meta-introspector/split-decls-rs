macro_rules! helper_no_refs {
    () => {
        # [doc = " Like `helper!` but without reference types."] macro_rules ! helper_no_refs { ($ callback : ident , $ ($ input : tt) *) => { $ callback ! ([proc_macro ::] => $ ($ input) *) ; # [cfg (feature = "proc-macro2")] $ callback ! ([proc_macro2 ::] => $ ($ input) *) ; } ; }
    };
}

helper_no_refs!();