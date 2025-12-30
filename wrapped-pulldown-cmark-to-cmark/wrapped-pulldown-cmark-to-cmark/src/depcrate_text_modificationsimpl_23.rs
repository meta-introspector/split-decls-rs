// Generated macro for impl_23 (impl)
macro_rules! Depcrate_text_modificationsimpl_23 {
() => {
// Module: crate::text_modifications
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : fmt :: Display > fmt :: Display for Repeated < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Repeated (content , count) = self ; for _ in 0 .. * count { T :: fmt (content , f) ? ; } Ok (()) } }
};
}
