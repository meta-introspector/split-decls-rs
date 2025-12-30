// Generated macro for Runner (struct)
macro_rules! Depcrate_tests_packedpairRunner {
() => {
// Module: crate::tests::packedpair
// Provides: {"Runner"}
// Dependencies: {}
# [doc = " Runs a host of \"packed pair\" search tests."] # [doc = ""] # [doc = " These tests specifically look for the occurrence of a possible substring"] # [doc = " match based on a pair of bytes matching at the right offsets."] pub (crate) struct Runner { fwd : Option < Box < dyn FnMut (& [u8] , & [u8] , u8 , u8) -> Option < Option < usize > > + 'static , > , > , }
};
}
