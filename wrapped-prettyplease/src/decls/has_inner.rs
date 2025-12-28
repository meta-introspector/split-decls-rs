macro_rules! has_inner {
    () => {
        pub fn has_inner (attrs : & [Attribute]) -> bool { for attr in attrs { if let AttrStyle :: Inner (_) = attr . style { return true ; } } false }
    };
}

has_inner!()