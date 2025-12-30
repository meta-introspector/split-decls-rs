// Generated macro for impl_86 (impl)
macro_rules! Depcrate_secret_keyimpl_86 {
() => {
// Module: crate::secret_key
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "pem")] impl FromStr for SecretKey { type Err = Error ; fn from_str (s : & str) -> core :: result :: Result < Self , Error > { Self :: from_pkcs8_pem (s) . map_err (| _ | Error) } }
};
}
