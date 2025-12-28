macro_rules! deps {
    () => {
        IdentClass!();
        RootDatabase!();
        Definition!();
    };
}

macro_rules! get_definition {
    () => {
        deps!();
        pub fn get_definition (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken ,) -> Option < Definition > { for token in sema . descend_into_macros_exact (token) { let def = IdentClass :: classify_token (sema , & token) . map (IdentClass :: definitions_no_ops) ; if let Some (& [x]) = def . as_deref () { return Some (x) ; } } None }
    };
}

get_definition!()