// Generated macro for impl_54 (impl)
macro_rules! Depcrate_axisimpl_54 {
() => {
// Module: crate::axis
// Provides: {"impl_54"}
// Dependencies: {}
impl Set < Range > for Properties { # [doc = " Changes the range of the axis that will be shown"] # [doc = ""] # [doc = " **Note** All axes are auto-scaled by default"] fn set (& mut self , range : Range) -> & mut Properties { self . hidden = false ; match range { Range :: Auto => self . range = None , Range :: Limits (low , high) => self . range = Some ((low , high)) , } self } }
};
}
