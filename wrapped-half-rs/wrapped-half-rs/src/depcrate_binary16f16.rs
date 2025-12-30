// Generated macro for f16 (struct)
macro_rules! Depcrate_binary16f16 {
() => {
// Module: crate::binary16
// Provides: {"f16"}
// Dependencies: {}
# [doc = " A 16-bit floating point type implementing the IEEE 754-2008 standard [`binary16`] a.k.a \"half\""] # [doc = " format."] # [doc = ""] # [doc = " This 16-bit floating point type is intended for efficient storage where the full range and"] # [doc = " precision of a larger floating point value is not required."] # [doc = ""] # [doc = " [`binary16`]: https://en.wikipedia.org/wiki/Half-precision_floating-point_format"] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Default)] # [repr (transparent)] # [cfg_attr (feature = "serde" , derive (Serialize))] # [cfg_attr (feature = "rkyv" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize))] # [cfg_attr (feature = "rkyv" , rkyv (resolver = F16Resolver))] # [cfg_attr (feature = "bytemuck" , derive (Zeroable , Pod))] # [cfg_attr (kani , derive (kani :: Arbitrary))] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (FromBytes , Immutable , IntoBytes , KnownLayout)] pub struct f16 (u16) ;
};
}
