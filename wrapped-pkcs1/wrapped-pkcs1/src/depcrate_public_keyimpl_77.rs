// Generated macro for impl_77 (impl)
macro_rules! Depcrate_public_keyimpl_77 {
() => {
// Module: crate::public_key
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < & RsaPublicKey < '_ > > for Document { type Error = Error ; fn try_from (spki : & RsaPublicKey < '_ >) -> Result < Document > { Ok (Self :: encode_msg (spki) ?) } }
};
}
