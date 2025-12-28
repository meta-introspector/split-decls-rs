macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl PartialEq for SyntaxText { fn eq (& self , other : & SyntaxText) -> bool { if self . range . len () != other . range . len () { return false ; } let mut lhs = self . tokens_with_ranges () ; let mut rhs = other . tokens_with_ranges () ; zip_texts (& mut lhs , & mut rhs) . is_none () && lhs . all (| it | it . 1 . is_empty ()) && rhs . all (| it | it . 1 . is_empty ()) } }
    };
}

impl_94!()