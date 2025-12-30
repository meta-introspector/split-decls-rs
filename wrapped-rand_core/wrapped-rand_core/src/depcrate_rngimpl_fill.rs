// Generated macro for impl_fill (macro)
macro_rules! Depcrate_rngimpl_fill {
() => {
// Module: crate::rng
// Provides: {"impl_fill"}
// Dependencies: {}
# [doc = " Implement `Fill` for given type `$t`."] # [doc = ""] # [doc = " # Safety"] # [doc = " All bit patterns of `[u8; size_of::<$t>()]` must represent values of `$t`."] macro_rules ! impl_fill { () => { } ; (to_le ! plain $ x : ident) => { $ x . to_le () } ; (to_le ! wrapping $ x : ident) => { Wrapping ($ x . 0 . to_le ()) } ; (fill_slice ! $ t : ty , $ to_le : tt) => { fn fill_slice < R : Rng + ? Sized > (this : & mut [Self] , rng : & mut R) { if this . len () > 0 { let size = mem :: size_of_val (this) ; rng . fill_bytes (unsafe { slice :: from_raw_parts_mut (this . as_mut_ptr () as * mut u8 , size) }) ; for x in this { * x = impl_fill ! (to_le ! $ to_le x) ; } } } } ; ($ t : ty) => { { __unsafe () ; impl Fill for $ t { impl_fill ! (fill_slice ! $ t , plain) ; } impl Fill for Wrapping <$ t > { impl_fill ! (fill_slice ! $ t , wrapping) ; } } } ; ($ t : ty , $ ($ tt : ty ,) *) => { { impl_fill ! ($ t) ; impl_fill ! ($ ($ tt ,) *) ; } } }
};
}
