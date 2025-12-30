// Generated macro for impl_bytemuck (module)
macro_rules! Depcrateimpl_bytemuck {
() => {
// Module: crate
// Provides: {"impl_bytemuck"}
// Dependencies: {}
# [cfg (feature = "bytemuck")] mod impl_bytemuck { use super :: { FloatCore , NotNan , OrderedFloat } ; use bytemuck :: { AnyBitPattern , CheckedBitPattern , NoUninit , Pod , TransparentWrapper , Zeroable } ; unsafe impl < T : Zeroable > Zeroable for OrderedFloat < T > { } unsafe impl < T : Zeroable > Zeroable for NotNan < T > { } unsafe impl < T : Pod > Pod for OrderedFloat < T > { } unsafe impl < T : NoUninit > NoUninit for NotNan < T > { } unsafe impl < T : FloatCore + AnyBitPattern > CheckedBitPattern for NotNan < T > { type Bits = T ; fn is_valid_bit_pattern (bits : & Self :: Bits) -> bool { ! bits . is_nan () } } unsafe impl < T > TransparentWrapper < T > for OrderedFloat < T > { } # [test] fn test_not_nan_bit_pattern () { use bytemuck :: checked :: { try_cast , CheckedCastError } ; let nan = f64 :: NAN ; assert_eq ! (try_cast ::< f64 , NotNan < f64 >> (nan) , Err (CheckedCastError :: InvalidBitPattern) ,) ; let pi = core :: f64 :: consts :: PI ; assert ! (try_cast ::< f64 , NotNan < f64 >> (pi) . is_ok ()) ; } }
};
}
