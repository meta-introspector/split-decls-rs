// Generated macro for wrapping_bounded (function)
macro_rules! Depcrate_boundswrapping_bounded {
() => {
// Module: crate::bounds
// Provides: {"wrapping_bounded"}
// Dependencies: {}
# [test] fn wrapping_bounded () { macro_rules ! test_wrapping_bounded { ($ ($ t : ty) +) => { $ (assert_eq ! (< Wrapping <$ t > as Bounded >:: min_value () . 0 , <$ t >:: min_value ()) ; assert_eq ! (< Wrapping <$ t > as Bounded >:: max_value () . 0 , <$ t >:: max_value ()) ;) + } ; } test_wrapping_bounded ! (usize u8 u16 u32 u64 isize i8 i16 i32 i64) ; }
};
}
