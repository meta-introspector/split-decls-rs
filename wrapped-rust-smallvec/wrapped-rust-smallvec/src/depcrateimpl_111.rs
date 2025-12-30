// Generated macro for impl_111 (impl)
macro_rules! Depcrateimpl_111 {
() => {
// Module: crate
// Provides: {"impl_111"}
// Dependencies: {}
impl TaggedLen { # [inline] pub const fn new (len : usize , on_heap : bool , is_zst : bool) -> Self { if is_zst { debug_assert ! (! on_heap) ; TaggedLen (len) } else { debug_assert ! (len < isize :: MAX as usize) ; TaggedLen ((len << 1) | on_heap as usize) } } # [inline] # [must_use] pub const fn on_heap (self , is_zst : bool) -> bool { if is_zst { false } else { (self . 0 & 1_usize) == 1 } } # [inline] pub const fn value (self , is_zst : bool) -> usize { if is_zst { self . 0 } else { self . 0 >> 1 } } }
};
}
