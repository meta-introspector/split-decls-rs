// Generated macro for required_hashes (function)
macro_rules! Depcrate_utilsrequired_hashes {
() => {
// Module: crate::utils
// Provides: {"required_hashes"}
// Dependencies: {}
# [doc = " Calculate the number of hashes required for a raw string containing `s`"] pub (crate) fn required_hashes (s : & str) -> usize { let mut res = 0usize ; for idx in s . match_indices ('"') . map (| (i , _) | i) { let (_ , sub) = s . split_at (idx + 1) ; let n_hashes = sub . chars () . take_while (| c | * c == '#') . count () ; res = res . max (n_hashes + 1) } res }
};
}
