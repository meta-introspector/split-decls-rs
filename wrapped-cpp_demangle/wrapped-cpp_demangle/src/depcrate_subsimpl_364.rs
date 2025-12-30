// Generated macro for impl_364 (impl)
macro_rules! Depcrate_subsimpl_364 {
() => {
// Module: crate::subs
// Provides: {"impl_364"}
// Dependencies: {}
impl fmt :: Debug for SubstitutionTable { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . pad ("SubstitutionTable ") ? ; f . debug_map () . entries (self . substitutions . iter () . enumerate ()) . finish () ? ; f . pad (" non_substitutions ") ? ; f . debug_map () . entries (self . non_substitutions . iter () . enumerate ()) . finish () } }
};
}
