// Generated macro for impl_30 (impl)
macro_rules! Depcrate_array_stringimpl_30 {
() => {
// Module: crate::array_string
// Provides: {"impl_30"}
// Dependencies: {}
impl < const CAP : usize > DerefMut for ArrayString < CAP > { # [inline] fn deref_mut (& mut self) -> & mut str { unsafe { let len = self . len () ; let sl = slice :: from_raw_parts_mut (self . as_mut_ptr () , len) ; str :: from_utf8_unchecked_mut (sl) } } }
};
}
