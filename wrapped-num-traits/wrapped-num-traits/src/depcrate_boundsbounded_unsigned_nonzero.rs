// Generated macro for bounded_unsigned_nonzero (function)
macro_rules! Depcrate_boundsbounded_unsigned_nonzero {
() => {
// Module: crate::bounds
// Provides: {"bounded_unsigned_nonzero"}
// Dependencies: {}
# [test] fn bounded_unsigned_nonzero () { macro_rules ! test_bounded_impl_unsigned_nonzero { ($ t : ty , $ base_ty : ty) => { assert_eq ! (<$ t as Bounded >:: min_value () . get () , 1) ; assert_eq ! (<$ t as Bounded >:: max_value () . get () , <$ base_ty >:: MAX) ; } ; } test_bounded_impl_unsigned_nonzero ! (NonZeroUsize , usize) ; test_bounded_impl_unsigned_nonzero ! (NonZeroU8 , u8) ; test_bounded_impl_unsigned_nonzero ! (NonZeroU16 , u16) ; test_bounded_impl_unsigned_nonzero ! (NonZeroU32 , u32) ; test_bounded_impl_unsigned_nonzero ! (NonZeroU64 , u64) ; test_bounded_impl_unsigned_nonzero ! (NonZeroU128 , u128) ; }
};
}
