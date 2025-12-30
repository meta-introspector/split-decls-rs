// Generated macro for impl_70 (impl)
macro_rules! Depcrate_public_keyimpl_70 {
() => {
// Module: crate::public_key
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (feature = "pem")] impl FromStr for PublicKey { type Err = Error ; fn from_str (s : & str) -> Result < Self , Error > { Self :: from_public_key_pem (s) . map_err (| _ | Error) } }
};
}
