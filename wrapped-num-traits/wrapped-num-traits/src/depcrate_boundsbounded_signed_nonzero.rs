// Generated macro for bounded_signed_nonzero (function)
macro_rules! Depcrate_boundsbounded_signed_nonzero {
() => {
// Module: crate::bounds
// Provides: {"bounded_signed_nonzero"}
// Dependencies: {}
# [test] fn bounded_signed_nonzero () { macro_rules ! test_bounded_impl_signed_nonzero { ($ t : ty , $ base_ty : ty) => { assert_eq ! (<$ t as Bounded >:: min_value () . get () , <$ base_ty >:: MIN) ; assert_eq ! (<$ t as Bounded >:: max_value () . get () , <$ base_ty >:: MAX) ; } ; } test_bounded_impl_signed_nonzero ! (NonZeroIsize , isize) ; test_bounded_impl_signed_nonzero ! (NonZeroI8 , i8) ; test_bounded_impl_signed_nonzero ! (NonZeroI16 , i16) ; test_bounded_impl_signed_nonzero ! (NonZeroI32 , i32) ; test_bounded_impl_signed_nonzero ! (NonZeroI64 , i64) ; test_bounded_impl_signed_nonzero ! (NonZeroI128 , i128) ; }
};
}
