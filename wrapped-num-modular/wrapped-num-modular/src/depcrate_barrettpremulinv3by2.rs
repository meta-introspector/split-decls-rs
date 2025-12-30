// Generated macro for PreMulInv3by2 (struct)
macro_rules! Depcrate_barrettPreMulInv3by2 {
() => {
// Module: crate::barrett
// Provides: {"PreMulInv3by2"}
// Dependencies: {}
# [doc = " A wrapper of [Normalized3by2Divisor] that can be used as a [Reducer]"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct PreMulInv3by2 < T , D > { div : Normalized3by2Divisor < T , D > , shift : u32 , }
};
}
