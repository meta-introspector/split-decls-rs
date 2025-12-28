macro_rules! has_attr {
    () => {
        fn has_attr (attrs : & [syn :: Attribute] , name : & str) -> bool { attrs . iter () . any (| a | { if let Ok (i) = a . parse_args :: < Ident > () { if i == name { return true ; } } false }) }
    };
}

has_attr!()