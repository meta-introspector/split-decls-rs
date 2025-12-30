// Generated macro for impl_324 (impl)
macro_rules! Depcrate_no_stdimpl_324 {
() => {
// Module: crate::no_std
// Provides: {"impl_324"}
// Dependencies: {}
impl Read for & [u8] { # [inline (always)] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { let length = self . len () . min (buf . len ()) ; let (left , right) = self . split_at (length) ; buf [.. length] . copy_from_slice (left) ; * self = right ; Ok (length) } }
};
}
