// Generated macro for Finder (struct)
macro_rules! Depcrate_arch_aarch64_neon_packedpairFinder {
() => {
// Module: crate::arch::aarch64::neon::packedpair
// Provides: {"Finder"}
// Dependencies: {}
# [doc = " A \"packed pair\" finder that uses 128-bit vector operations."] # [doc = ""] # [doc = " This finder picks two bytes that it believes have high predictive power"] # [doc = " for indicating an overall match of a needle. Depending on whether"] # [doc = " `Finder::find` or `Finder::find_prefilter` is used, it reports offsets"] # [doc = " where the needle matches or could match. In the prefilter case, candidates"] # [doc = " are reported whenever the [`Pair`] of bytes given matches."] # [derive (Clone , Copy , Debug)] pub struct Finder (packedpair :: Finder < uint8x16_t >) ;
};
}
