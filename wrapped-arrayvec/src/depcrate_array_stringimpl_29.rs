// Generated macro for impl_29 (impl)
macro_rules! Depcrate_array_stringimpl_29 {
() => {
// Module: crate::array_string
// Provides: {"impl_29"}
// Dependencies: {}
impl < const CAP : usize > Deref for ArrayString < CAP > { type Target = str ; # [inline] fn deref (& self) -> & str { unsafe { let sl = slice :: from_raw_parts (self . as_ptr () , self . len ()) ; str :: from_utf8_unchecked (sl) } } }
};
}
