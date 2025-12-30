// Generated macro for normalize_scheme (function)
macro_rules! Depcrate_normalizenormalize_scheme {
() => {
// Module: crate::normalize
// Provides: {"normalize_scheme"}
// Dependencies: {}
# [doc = " Writes the normalized scheme."] pub (crate) fn normalize_scheme (f : & mut fmt :: Formatter < '_ > , scheme : & str) -> fmt :: Result { scheme . chars () . map (| c | c . to_ascii_lowercase ()) . try_for_each (| c | f . write_char (c)) }
};
}
