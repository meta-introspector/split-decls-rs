// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl ReadLike for ReadBytes < '_ > { type Error = Infallible ; fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { let n = self . 0 . len () . min (buf . len ()) ; buf [.. n] . copy_from_slice (& self . 0 [.. n]) ; self . 0 = & self . 0 [n ..] ; Ok (n) } }
};
}
