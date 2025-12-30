// Generated macro for impl_21 (impl)
macro_rules! Depcrate_specimpl_21 {
() => {
// Module: crate::spec
// Provides: {"impl_21"}
// Dependencies: {}
# [doc = " Conversion"] impl RefSpecRef < '_ > { # [doc = " Convert this ref into a standalone, owned copy."] pub fn to_owned (& self) -> RefSpec { RefSpec { mode : self . mode , op : self . op , src : self . src . map (ToOwned :: to_owned) , dst : self . dst . map (ToOwned :: to_owned) , } } }
};
}
