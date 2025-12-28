macro_rules! deps {
    () => {
        DeriveWriter!();
        TypeName!();
        Config!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl DeriveWriter { pub fn new (config : & Config , type_name : TypeName) -> Self { let mut derive = BTreeSet :: new () ; derive . extend (config . derive . get (type_name)) ; Self (derive) } pub fn extend < I , S > (& mut self , iter : I) where I : IntoIterator < Item = S > , S : AsRef < str > + ToString , { self . 0 . extend (iter . into_iter () . map (| s | s . to_string ())) ; } }
    };
}

impl_25!();