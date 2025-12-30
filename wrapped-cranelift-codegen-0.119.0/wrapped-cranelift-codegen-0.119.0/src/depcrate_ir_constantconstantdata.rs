// Generated macro for ConstantData (struct)
macro_rules! Depcrate_ir_constantConstantData {
() => {
// Module: crate::ir::constant
// Provides: {"ConstantData"}
// Dependencies: {}
# [doc = " This type describes the actual constant data. Note that the bytes stored in this structure are"] # [doc = " expected to be in little-endian order; this is due to ease-of-use when interacting with"] # [doc = " WebAssembly values, which are [little-endian by design]."] # [doc = ""] # [doc = " [little-endian by design]: https://github.com/WebAssembly/design/blob/master/Portability.md"] # [derive (Clone , Hash , Eq , PartialEq , Debug , Default , PartialOrd , Ord)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct ConstantData (Vec < u8 >) ;
};
}
