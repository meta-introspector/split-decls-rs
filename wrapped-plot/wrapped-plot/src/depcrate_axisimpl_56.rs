// Generated macro for impl_56 (impl)
macro_rules! Depcrate_axisimpl_56 {
() => {
// Module: crate::axis
// Provides: {"impl_56"}
// Dependencies: {}
impl Set < ScaleFactor > for Properties { # [doc = " Changes the *scale factor* of the axis."] # [doc = ""] # [doc = " All the data plotted against this axis will have its corresponding coordinate scaled with"] # [doc = " this factor before being plotted."] # [doc = ""] # [doc = " **Note** The default scale factor is `1`."] fn set (& mut self , factor : ScaleFactor) -> & mut Properties { self . scale_factor = factor . 0 ; self } }
};
}
