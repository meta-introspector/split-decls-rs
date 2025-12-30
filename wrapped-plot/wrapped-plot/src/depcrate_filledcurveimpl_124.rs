// Generated macro for impl_124 (impl)
macro_rules! Depcrate_filledcurveimpl_124 {
() => {
// Module: crate::filledcurve
// Provides: {"impl_124"}
// Dependencies: {}
impl Set < Opacity > for Properties { # [doc = " Changes the opacity of the fill color"] # [doc = ""] # [doc = " **Note** By default, the fill color is totally opaque (`opacity = 1.0`)"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `opacity` is outside the range `[0, 1]`"] fn set (& mut self , opacity : Opacity) -> & mut Properties { self . opacity = Some (opacity . 0) ; self } }
};
}
