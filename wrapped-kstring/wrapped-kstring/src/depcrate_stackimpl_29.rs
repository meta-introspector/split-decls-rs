// Generated macro for impl_29 (impl)
macro_rules! Depcrate_stackimpl_29 {
() => {
// Module: crate::stack
// Provides: {"impl_29"}
// Dependencies: {}
impl < const CAPACITY : usize > StrBuffer < CAPACITY > { # [inline] # [cfg (feature = "unsafe")] pub (crate) unsafe fn new_unchecked (s : & str) -> Self { let len = s . len () ; debug_assert ! (len <= CAPACITY) ; let mut buffer = Self :: default () ; buffer . 0 . get_unchecked_mut (.. len) . copy_from_slice (s . as_bytes ()) ; buffer } # [inline] # [cfg (feature = "unsafe")] pub (crate) unsafe fn as_str_unchecked (& self , len : usize) -> & str { let slice = self . 0 . get_unchecked (.. len) ; std :: str :: from_utf8_unchecked (slice) } # [inline] # [cfg (feature = "unsafe")] pub (crate) unsafe fn as_mut_str_unchecked (& mut self , len : usize) -> & mut str { let slice = self . 0 . get_unchecked_mut (.. len) ; std :: str :: from_utf8_unchecked_mut (slice) } }
};
}
