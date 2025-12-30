// Generated macro for impl_445 (impl)
macro_rules! Depcrate_ex_dataimpl_445 {
() => {
// Module: crate::ex_data
// Provides: {"impl_445"}
// Dependencies: {}
impl < T , U > Index < T , U > { # [doc = " Creates an `Index` from a raw integer index."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that the index correctly maps to a `U` value stored in a `T`."] pub unsafe fn from_raw (idx : c_int) -> Index < T , U > { Index (idx , PhantomData) } # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
