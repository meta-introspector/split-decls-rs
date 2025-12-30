// Generated macro for impl_1462 (impl)
macro_rules! Depcrate_stringimpl_1462 {
() => {
// Module: crate::string
// Provides: {"impl_1462"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < I > ops :: Index < I > for String where I : slice :: SliceIndex < str > , { type Output = I :: Output ; # [inline] fn index (& self , index : I) -> & I :: Output { index . index (self . as_str ()) } }
};
}
