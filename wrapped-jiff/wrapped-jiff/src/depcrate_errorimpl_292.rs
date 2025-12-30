// Generated macro for impl_292 (impl)
macro_rules! Depcrate_errorimpl_292 {
() => {
// Module: crate::error
// Provides: {"impl_292"}
// Dependencies: {}
impl core :: fmt :: Display for RangeError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (feature = "alloc")] { let RangeError { what , given , min , max } = * self ; write ! (f , "parameter '{what}' with value {given} \
                 is not in the required range of {min}..={max}" ,) } # [cfg (not (feature = "alloc"))] { let RangeError { what } = * self ; write ! (f , "parameter '{what}' is not in the required range") } } }
};
}
