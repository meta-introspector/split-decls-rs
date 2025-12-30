// Generated macro for DecompressionMatcherBuilder (struct)
macro_rules! Depcrate_decompressDecompressionMatcherBuilder {
() => {
// Module: crate::decompress
// Provides: {"DecompressionMatcherBuilder"}
// Dependencies: {}
# [doc = " A builder for a matcher that determines which files get decompressed."] # [derive (Clone , Debug)] pub struct DecompressionMatcherBuilder { # [doc = " The commands for each matching glob."] commands : Vec < DecompressionCommand > , # [doc = " Whether to include the default matching rules."] defaults : bool , }
};
}
