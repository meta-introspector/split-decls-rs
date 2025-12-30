// Generated macro for normalize_query (function)
macro_rules! Depcrate_normalizenormalize_query {
() => {
// Module: crate::normalize
// Provides: {"normalize_query"}
// Dependencies: {}
# [doc = " Writes the normalized query without the '?' prefix."] pub (crate) fn normalize_query < S : Spec > (f : & mut fmt :: Formatter < '_ > , query : & str) -> fmt :: Result { PctCaseNormalized :: < S > :: new (query) . fmt (f) }
};
}
