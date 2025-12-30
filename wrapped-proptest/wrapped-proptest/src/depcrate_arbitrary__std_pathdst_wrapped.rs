// Generated macro for dst_wrapped (macro)
macro_rules! Depcrate_arbitrary__std_pathdst_wrapped {
() => {
// Module: crate::arbitrary::_std::path
// Provides: {"dst_wrapped"}
// Dependencies: {}
macro_rules ! dst_wrapped { ($ ($ w : ident) ,*) => { $ (# [doc = " This implementation is identical to [the `Arbitrary` implementation for"] # [doc = " `PathBuf`](trait.Arbitrary.html#impl-Arbitrary-for-PathBuf)."] impl Arbitrary for $ w < Path > { type Parameters = PathParams ; type Strategy = MapInto < StrategyFor < PathBuf >, Self >; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { any_with ::< PathBuf > (args) . prop_map_into () } }) * } }
};
}
