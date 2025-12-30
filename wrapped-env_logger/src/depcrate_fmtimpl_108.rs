// Generated macro for impl_108 (impl)
macro_rules! Depcrate_fmtimpl_108 {
() => {
// Module: crate::fmt
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (feature = "color")] impl < T : Display > Display for StyledValue < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let style = self . style ; write ! (f , "{style}") ? ; self . value . fmt (f) ? ; write ! (f , "{style:#}") ? ; Ok (()) } }
};
}
