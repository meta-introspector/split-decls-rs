macro_rules! deps {
    () => {
        Ident!();
        Cursor!();
        Reject!();
        PResult!();
    };
}

macro_rules! ident {
    () => {
        deps!();
        fn ident (input : Cursor) -> PResult < crate :: Ident > { if ["r\"" , "r#\"" , "r##" , "b\"" , "b\'" , "br\"" , "br#" , "c\"" , "cr\"" , "cr#" ,] . iter () . any (| prefix | input . starts_with (prefix)) { Err (Reject) } else { ident_any (input) } }
    };
}

ident!();