macro_rules! deps {
    () => {
        RawAttribute!();
    };
}

macro_rules! check {
    () => {
        deps!();
        pub (crate) fn check (ctxt : & str , allowed : & [& dyn RawAttribute] , attrs : & [Attribute]) { let mut seen = HashSet :: new () ; for (value_key , _) in attrs . iter () . filter_map (| attr | sval_attr (ctxt , attr)) . flatten () { let mut is_valid_attr = false ; for attr in allowed { let attr_key = attr . key () ; if value_key . is_ident (attr_key) { is_valid_attr = true ; if ! seen . insert (attr_key) { panic ! ("duplicate attribute `{}` on {}" , quote ! (# value_key) , ctxt) ; } } } if ! is_valid_attr { panic ! ("unsupported attribute `{}` on {}" , quote ! (# value_key) , ctxt) ; } } }
    };
}

check!()