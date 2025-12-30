// Generated macro for impl_561 (impl)
macro_rules! Depcrate_arrayimpl_561 {
() => {
// Module: crate::array
// Provides: {"impl_561"}
// Dependencies: {}
impl < S , T > UniformArrayStrategy < S , T > { # [doc = " Directly create a `UniformArrayStrategy`."] # [doc = ""] # [doc = " This is only intended for advanced use, since the only way to specify"] # [doc = " the array size is with the turbofish operator and explicitly naming the"] # [doc = " type of the values in the array and the strategy itself."] # [doc = ""] # [doc = " Prefer the `uniformXX` functions at module-level unless something"] # [doc = " precludes their use."] pub fn new (strategy : S) -> Self { UniformArrayStrategy { strategy , _marker : PhantomData , } } }
};
}
