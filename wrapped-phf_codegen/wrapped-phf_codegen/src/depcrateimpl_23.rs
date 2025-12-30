// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , T : Hash + PhfHash + Eq + FmtConst > OrderedSet < 'a , T > { # [doc = " Constructs a new `phf::OrderedSet` builder."] pub fn new () -> Self { OrderedSet { map : OrderedMap :: new () , } } # [doc = " Set the path to the `phf` crate from the global namespace"] pub fn phf_path (& mut self , path : impl Into < Cow < 'a , str > >) -> & mut Self { self . map . phf_path (path) ; self } # [doc = " Adds an entry to the builder."] pub fn entry (& mut self , entry : T) -> & mut Self { self . map . entry (entry , "()") ; self } # [doc = " Calculate the hash parameters and return a struct implementing"] # [doc = " [`Display`](::std::fmt::Display) which will print the constructed"] # [doc = " `phf::OrderedSet`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if there are any duplicate keys."] pub fn build (& self) -> DisplayOrderedSet < '_ , T > { DisplayOrderedSet { inner : self . map . build () , } } }
};
}
