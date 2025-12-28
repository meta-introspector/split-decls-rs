macro_rules! deps {
    () => {
        Reader!();
        TypeName!();
        Filter!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Filter { # [track_caller] pub fn new (reader : & Reader , include : & [& str] , exclude : & [& str]) -> Self { let mut rules = vec ! [] ; for filter in include { push_filter (reader , & mut rules , filter , true) ; } for filter in exclude { push_filter (reader , & mut rules , filter , false) } debug_assert ! (! rules . is_empty ()) ; rules . sort_unstable_by (| left , right | { let left = (left . 0 . len () , ! left . 1) ; let right = (right . 0 . len () , ! right . 1) ; left . cmp (& right) . reverse () }) ; Self (rules) } pub fn includes_namespace (& self , namespace : & str) -> bool { for rule in & self . 0 { if rule . 1 { if namespace_starts_with (& rule . 0 , namespace) { return true ; } if namespace_starts_with (namespace , & rule . 0) { return true ; } } else { if namespace_starts_with (namespace , & rule . 0) { return false ; } } } false } pub fn includes_type_name (& self , name : TypeName) -> bool { for rule in & self . 0 { if match_type_name (& rule . 0 , name . namespace () , name . name ()) { return rule . 1 ; } } false } pub fn excludes_type_name (& self , name : TypeName) -> bool { for rule in & self . 0 { if match_type_name (& rule . 0 , name . namespace () , name . name ()) { return ! rule . 1 ; } } false } }
    };
}

impl_29!()