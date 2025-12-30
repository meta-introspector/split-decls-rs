// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: alloc :: System ; # [global_allocator] static A : GgAlloc < System > = GgAlloc :: new (System) ; fn assert_above_2g (ptr : * mut u8) { assert ! (pointer_above_2g (ptr) , "{:p}" , ptr ,) ; } # [test] fn alloc_string () { let s = format ! ("allocating a string!") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; } # [test] fn alloc_one_byte () { let s = format ! ("1") ; assert ! (pointer_above_2g (s . as_str () as * const _ as * mut _) , "{:p}" , s . as_str ()) ; } # [test] fn alloc_many_bytes () { let s = format ! ("1") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("12") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("123") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("1234") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("12345") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("123456") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("1234567") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("12345678") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; let s = format ! ("123456789") ; assert_above_2g (s . as_str () as * const _ as * mut _) ; } }
};
}
