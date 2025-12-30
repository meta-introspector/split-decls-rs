// Generated macro for impl_11 (impl)
macro_rules! Depcrate_decompressimpl_11 {
() => {
// Module: crate::decompress
// Provides: {"impl_11"}
// Dependencies: {}
impl DecompressionMatcher { # [doc = " Create a new matcher with default rules."] # [doc = ""] # [doc = " To add more matching rules, build a matcher with"] # [doc = " [`DecompressionMatcherBuilder`]."] pub fn new () -> DecompressionMatcher { DecompressionMatcherBuilder :: new () . build () . expect ("built-in matching rules should always compile") } # [doc = " Return a pre-built command based on the given file path that can"] # [doc = " decompress its contents. If no such decompressor is known, then this"] # [doc = " returns `None`."] # [doc = ""] # [doc = " If there are multiple possible commands matching the given path, then"] # [doc = " the command added last takes precedence."] pub fn command < P : AsRef < Path > > (& self , path : P) -> Option < Command > { if let Some (i) = self . globs . matches (path) . into_iter () . next_back () { let decomp_cmd = & self . commands [i] ; let mut cmd = Command :: new (& decomp_cmd . bin) ; cmd . args (& decomp_cmd . args) ; return Some (cmd) ; } None } # [doc = " Returns true if and only if the given file path has at least one"] # [doc = " matching command to perform decompression on."] pub fn has_command < P : AsRef < Path > > (& self , path : P) -> bool { self . globs . is_match (path) } }
};
}
