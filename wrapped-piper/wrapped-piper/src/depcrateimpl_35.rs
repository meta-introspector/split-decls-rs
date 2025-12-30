// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl WriteLike for WriteBytes < '_ > { type Error = Infallible ; fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { let n = self . 0 . len () . min (buf . len ()) ; self . 0 [.. n] . copy_from_slice (& buf [.. n]) ; # [allow (clippy :: mem_replace_with_default)] { let slice = mem :: replace (& mut self . 0 , & mut []) ; self . 0 = & mut slice [n ..] ; } Ok (n) } }
};
}
