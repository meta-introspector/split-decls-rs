// Generated macro for impl_55 (impl)
macro_rules! Depcrate_axisimpl_55 {
() => {
// Module: crate::axis
// Provides: {"impl_55"}
// Dependencies: {}
impl Set < Scale > for Properties { # [doc = " Sets the scale of the axis"] # [doc = ""] # [doc = " **Note** All axes use a linear scale by default"] fn set (& mut self , scale : Scale) -> & mut Properties { self . hidden = false ; match scale { Scale :: Linear => self . logarithmic = false , Scale :: Logarithmic => self . logarithmic = true , } self } }
};
}
