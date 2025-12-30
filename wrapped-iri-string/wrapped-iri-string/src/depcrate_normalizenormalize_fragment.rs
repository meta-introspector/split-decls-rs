// Generated macro for normalize_fragment (function)
macro_rules! Depcrate_normalizenormalize_fragment {
() => {
// Module: crate::normalize
// Provides: {"normalize_fragment"}
// Dependencies: {}
# [doc = " Writes the normalized query without the '#' prefix."] pub (crate) fn normalize_fragment < S : Spec > (f : & mut fmt :: Formatter < '_ > , fragment : & str ,) -> fmt :: Result { PctCaseNormalized :: < S > :: new (fragment) . fmt (f) }
};
}
