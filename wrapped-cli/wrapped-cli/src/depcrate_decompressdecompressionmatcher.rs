// Generated macro for DecompressionMatcher (struct)
macro_rules! Depcrate_decompressDecompressionMatcher {
() => {
// Module: crate::decompress
// Provides: {"DecompressionMatcher"}
// Dependencies: {}
# [doc = " A matcher for determining how to decompress files."] # [derive (Clone , Debug)] pub struct DecompressionMatcher { # [doc = " The set of globs to match. Each glob has a corresponding entry in"] # [doc = " `commands`. When a glob matches, the corresponding command should be"] # [doc = " used to perform out-of-process decompression."] globs : GlobSet , # [doc = " The commands for each matching glob."] commands : Vec < DecompressionCommand > , }
};
}
