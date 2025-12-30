// Generated macro for digest (function)
macro_rules! Depcrate_digestdigest {
() => {
// Module: crate::digest
// Provides: {"digest"}
// Dependencies: {}
# [doc = " Returns the digest of `data` using the given digest algorithm."] pub fn digest (algorithm : & 'static Algorithm , data : & [u8]) -> Digest { let cpu = cpu :: features () ; Digest :: compute_from (algorithm , data , cpu) . map_err (error :: erase :: < InputTooLongError >) . unwrap () }
};
}
