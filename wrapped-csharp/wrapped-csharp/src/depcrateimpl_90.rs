// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl Opts { pub fn build (& self) -> Box < dyn WorldGenerator > { Box :: new (world_generator :: CSharp { opts : self . clone () , .. world_generator :: CSharp :: default () }) } }
};
}
