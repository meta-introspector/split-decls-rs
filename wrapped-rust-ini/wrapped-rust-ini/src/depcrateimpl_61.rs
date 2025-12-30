// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > Ini { # [doc = " Immutable iterate though sections"] pub fn iter (& 'a self) -> SectionIter < 'a > { SectionIter { inner : self . sections . iter () , } } # [doc = " Mutable iterate though sections"] # [deprecated (note = "Use `iter_mut` instead!")] pub fn mut_iter (& 'a mut self) -> SectionIterMut < 'a > { self . iter_mut () } # [doc = " Mutable iterate though sections"] pub fn iter_mut (& 'a mut self) -> SectionIterMut < 'a > { SectionIterMut { inner : self . sections . iter_mut () , } } }
};
}
