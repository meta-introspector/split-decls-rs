// Generated macro for impl_86 (impl)
macro_rules! Depcrate_curveimpl_86 {
() => {
// Module: crate::curve
// Provides: {"impl_86"}
// Dependencies: {}
impl Set < LineWidth > for Properties { # [doc = " Changes the width of the line"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `width` is a non-positive value"] fn set (& mut self , lw : LineWidth) -> & mut Properties { let lw = lw . 0 ; assert ! (lw > 0.) ; self . linewidth = Some (lw) ; self } }
};
}
