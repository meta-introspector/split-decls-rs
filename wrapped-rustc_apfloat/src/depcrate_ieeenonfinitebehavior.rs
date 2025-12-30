// Generated macro for NonfiniteBehavior (enum)
macro_rules! Depcrate_ieeeNonfiniteBehavior {
() => {
// Module: crate::ieee
// Provides: {"NonfiniteBehavior"}
// Dependencies: {}
# [doc = " How the nonfinite values Inf and NaN are represented."] # [derive (Copy , Clone , PartialEq , Eq)] pub enum NonfiniteBehavior { # [doc = " Represents standard IEEE 754 behavior. A value is nonfinite if the"] # [doc = " exponent field is all 1s. In such cases, a value is Inf if the"] # [doc = " significand bits are all zero, and NaN otherwise"] IEEE754 , # [doc = " Only the Float8E5M2 has this behavior. There is no Inf representation. A"] # [doc = " value is NaN if the exponent field and the mantissa field are all 1s."] # [doc = " This behavior matches the FP8 E4M3 type described in"] # [doc = " <https://arxiv.org/abs/2209.05433>. We treat both signed and unsigned NaNs"] # [doc = " as non-signalling, although the paper does not state whether the NaN"] # [doc = " values are signalling or not."] NanOnly , }
};
}
