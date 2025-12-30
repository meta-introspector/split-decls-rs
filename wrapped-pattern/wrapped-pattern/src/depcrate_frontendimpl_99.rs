// Generated macro for impl_99 (impl)
macro_rules! Depcrate_frontendimpl_99 {
() => {
// Module: crate::frontend
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a , B , P > fmt :: Display for WriteablePattern < 'a , B , P > where B : PatternBackend , P : PlaceholderValueProvider < B :: PlaceholderKey < 'a > , Error = B :: Error < 'a > > , { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . try_write_to (f) . map (| _ | ()) } }
};
}
