// Generated macro for WnafScalar (struct)
macro_rules! Depcrate_wnafWnafScalar {
() => {
// Module: crate::wnaf
// Provides: {"WnafScalar"}
// Dependencies: {}
# [doc = " A \"w-ary non-adjacent form\" scalar, that uses precomputation to improve the speed of"] # [doc = " scalar multiplication."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " See [`WnafBase`] for usage examples."] # [derive (Clone , Debug)] pub struct WnafScalar < F : PrimeField , const WINDOW_SIZE : usize > { wnaf : Vec < i64 > , field : PhantomData < F > , }
};
}
