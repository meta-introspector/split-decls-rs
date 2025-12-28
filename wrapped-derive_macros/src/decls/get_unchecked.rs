macro_rules! deps {
    () => {
        SvalAttribute!();
    };
}

macro_rules! get_unchecked {
    () => {
        deps!();
        pub (crate) fn get_unchecked < T : SvalAttribute > (ctxt : & str , request : T , attrs : & [Attribute] ,) -> Option < T :: Result > { let request_key = request . key () ; for (value_key , value) in attrs . iter () . filter_map (| attr | sval_attr (ctxt , attr)) . flatten () { if value_key . is_ident (request_key) { return Some (request . try_from_expr (& value) . expect ("unexpected value")) ; } } None }
    };
}

get_unchecked!();