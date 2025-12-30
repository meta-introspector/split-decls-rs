// Generated macro for impl_105 (impl)
macro_rules! Depcrate_errorbarimpl_105 {
() => {
// Module: crate::errorbar
// Provides: {"impl_105"}
// Dependencies: {}
impl Set < LineWidth > for Properties { # [doc = " Changes the linewidth"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `lw` is a non-positive value"] fn set (& mut self , lw : LineWidth) -> & mut Properties { let lw = lw . 0 ; assert ! (lw > 0.) ; self . linewidth = Some (lw) ; self } }
};
}
