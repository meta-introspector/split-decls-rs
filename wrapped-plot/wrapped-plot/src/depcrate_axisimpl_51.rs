// Generated macro for impl_51 (impl)
macro_rules! Depcrate_axisimpl_51 {
() => {
// Module: crate::axis
// Provides: {"impl_51"}
// Dependencies: {}
impl Properties { # [doc = " Hides the axis"] # [doc = ""] # [doc = " **Note** The `TopX` and `RightY` axes are hidden by default"] pub fn hide (& mut self) -> & mut Properties { self . hidden = true ; self } # [doc = " Makes the axis visible"] # [doc = ""] # [doc = " **Note** The `BottomX` and `LeftY` axes are visible by default"] pub fn show (& mut self) -> & mut Properties { self . hidden = false ; self } }
};
}
