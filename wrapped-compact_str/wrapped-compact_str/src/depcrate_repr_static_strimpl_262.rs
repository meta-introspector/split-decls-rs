// Generated macro for impl_262 (impl)
macro_rules! Depcrate_repr_static_strimpl_262 {
() => {
// Module: crate::repr::static_str
// Provides: {"impl_262"}
// Dependencies: {}
impl StaticStr { # [inline] pub (crate) const fn new (text : & 'static str) -> Self { let mut discriminant = [0 ; DISCRIMINANT_SIZE] ; discriminant [DISCRIMINANT_SIZE - 1] = STATIC_STR_MASK ; Self { ptr : unsafe { ptr :: NonNull :: new_unchecked (text . as_ptr () as * mut _) } , len : text . len () , discriminant , } } # [rustversion :: attr (since (1.64) , const)] pub (super) fn get_text (& self) -> & 'static str { unsafe { str :: from_utf8_unchecked (slice :: from_raw_parts (self . ptr . as_ptr () , self . len)) } } # [doc = " # Safety"] # [doc = " * `len` bytes in the buffer must be valid UTF-8 and"] # [doc = " * `len` must be <= `self.get_text().len()`"] pub (super) unsafe fn set_len (& mut self , len : usize) { * self = Self :: new (& self . get_text () [.. len]) ; } }
};
}
