// Generated macro for impl_890 (impl)
macro_rules! Depcrate_tls_prfimpl_890 {
() => {
// Module: crate::tls_prf
// Provides: {"impl_890"}
// Dependencies: {}
impl < const L : usize > TryFrom < Secret > for [u8 ; L] { type Error = Unspecified ; fn try_from (value : Secret) -> Result < Self , Self :: Error > { if value . secret . len () != L { return Err (Unspecified) ; } let mut ret = [0u8 ; L] ; ret . copy_from_slice (& value . secret) ; Ok (ret) } }
};
}
