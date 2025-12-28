macro_rules! has_outer {
    () => {
        pub fn has_outer (attrs : & [Attribute]) -> bool { for attr in attrs { if let AttrStyle :: Outer = attr . style { return true ; } } false }
    };
}

has_outer!()