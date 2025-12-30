// Generated macro for impl_28 (impl)
macro_rules! Depcrate_stackimpl_28 {
() => {
// Module: crate::stack
// Provides: {"impl_28"}
// Dependencies: {}
impl < const CAPACITY : usize > StrBuffer < CAPACITY > { pub (crate) const fn empty () -> Self { let array = [0 ; CAPACITY] ; StrBuffer (array) } # [inline] pub (crate) fn new (s : & str) -> Self { let len = s . len () ; debug_assert ! (len <= CAPACITY) ; let mut buffer = Self :: default () ; if let Some (buffer) = buffer . 0 . get_mut (.. len) { buffer . copy_from_slice (s . as_bytes ()) ; } else { panic ! ("`{s}` is larger than capacity {CAPACITY}") ; } buffer } # [inline] # [cfg (not (feature = "unsafe"))] pub (crate) fn as_str (& self , len : usize) -> & str { let slice = self . 0 . get (.. len) . unwrap () ; std :: str :: from_utf8 (slice) . unwrap () } # [inline] # [cfg (not (feature = "unsafe"))] pub (crate) fn as_mut_str (& mut self , len : usize) -> & mut str { let slice = self . 0 . get_mut (.. len) . unwrap () ; std :: str :: from_utf8_mut (slice) . unwrap () } }
};
}
