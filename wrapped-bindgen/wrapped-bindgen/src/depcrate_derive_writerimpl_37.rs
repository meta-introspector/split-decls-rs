// Generated macro for impl_37 (impl)
macro_rules! Depcrate_derive_writerimpl_37 {
() => {
// Module: crate::derive_writer
// Provides: {"impl_37"}
// Dependencies: {}
impl DeriveWriter { pub fn new (config : & Config , type_name : TypeName) -> Self { let mut derive = BTreeSet :: new () ; derive . extend (config . derive . get (type_name)) ; Self (derive) } pub fn extend < I , S > (& mut self , iter : I) where I : IntoIterator < Item = S > , S : AsRef < str > + ToString , { self . 0 . extend (iter . into_iter () . map (| s | s . to_string ())) ; } }
};
}
