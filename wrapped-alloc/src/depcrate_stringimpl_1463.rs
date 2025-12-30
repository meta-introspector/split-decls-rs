// Generated macro for impl_1463 (impl)
macro_rules! Depcrate_stringimpl_1463 {
() => {
// Module: crate::string
// Provides: {"impl_1463"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < I > ops :: IndexMut < I > for String where I : slice :: SliceIndex < str > , { # [inline] fn index_mut (& mut self , index : I) -> & mut I :: Output { index . index_mut (self . as_mut_str ()) } }
};
}
