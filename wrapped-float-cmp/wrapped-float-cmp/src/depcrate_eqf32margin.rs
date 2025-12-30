// Generated macro for F32Margin (struct)
macro_rules! Depcrate_eqF32Margin {
() => {
// Module: crate::eq
// Provides: {"F32Margin"}
// Dependencies: {}
# [doc = " This type defines a margin within two `f32` values might be considered equal,"] # [doc = " and is intended as the associated type for the `ApproxEq` trait."] # [doc = ""] # [doc = " Two tests are used to determine approximate equality."] # [doc = ""] # [doc = " The first test considers two values approximately equal if they differ by <="] # [doc = " `epsilon`. This will only succeed for very small numbers. Note that it may"] # [doc = " succeed even if the parameters are of differing signs, straddling zero."] # [doc = ""] # [doc = " The second test considers how many ULPs (units of least precision, units in"] # [doc = " the last place, which is the integer number of floating-point representations"] # [doc = " that the parameters are separated by) different the parameters are and considers"] # [doc = " them approximately equal if this is <= `ulps`. For large floating-point numbers,"] # [doc = " an ULP can be a rather large gap, but this kind of comparison is necessary"] # [doc = " because floating-point operations must round to the nearest representable value"] # [doc = " and so larger floating-point values accumulate larger errors."] # [repr (C)] # [derive (Debug , Clone , Copy)] pub struct F32Margin { pub epsilon : f32 , pub ulps : i32 , }
};
}
