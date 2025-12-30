// Generated macro for impl_37 (impl)
macro_rules! Depcrate_fuzzersimpl_37 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_37"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `PointerLevelC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for PointerLevelC { fn arbitrary (g : & mut Gen) -> PointerLevelC { PointerLevelC { def : (0 .. gen_range (g , 0 , 16)) . map (| _ | "*") . collect :: < String > () , } } }
};
}
