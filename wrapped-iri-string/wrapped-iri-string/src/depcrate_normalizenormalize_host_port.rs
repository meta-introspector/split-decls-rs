// Generated macro for normalize_host_port (function)
macro_rules! Depcrate_normalizenormalize_host_port {
() => {
// Module: crate::normalize
// Provides: {"normalize_host_port"}
// Dependencies: {}
# [doc = " Writes the normalized host and port."] pub (crate) fn normalize_host_port < S : Spec > (f : & mut fmt :: Formatter < '_ > , host_port : & str ,) -> fmt :: Result { let host_port = host_port . strip_suffix (':') . unwrap_or (host_port) ; if is_ascii_only_host (host_port) { NormalizedAsciiOnlyHost :: new (host_port) . fmt (f) } else { PctCaseNormalized :: < S > :: new (host_port) . fmt (f) } }
};
}
