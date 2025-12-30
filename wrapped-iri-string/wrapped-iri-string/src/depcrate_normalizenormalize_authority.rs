// Generated macro for normalize_authority (function)
macro_rules! Depcrate_normalizenormalize_authority {
() => {
// Module: crate::normalize
// Provides: {"normalize_authority"}
// Dependencies: {}
# [doc = " Writes the normalized authority."] fn normalize_authority < S : Spec > (f : & mut fmt :: Formatter < '_ > , authority : & str) -> fmt :: Result { let host_port = match rfind_split_hole (authority , b'@') { Some ((userinfo , host_port)) => { PctCaseNormalized :: < S > :: new (userinfo) . fmt (f) ? ; f . write_char ('@') ? ; host_port } None => authority , } ; normalize_host_port :: < S > (f , host_port) }
};
}
