// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > SectionOccupiedEntry < 'a > { # [doc = " Into the first internal mutable properties"] pub fn into_mut (self) -> & 'a mut Properties { self . inner . into_mut () } # [doc = " Append a new section"] pub fn append (& mut self , prop : Properties) { self . inner . append (prop) ; } fn last_mut (& 'a mut self) -> & 'a mut Properties { self . inner . iter_mut () . next_back () . expect ("occupied section shouldn't have 0 property") } }
};
}
