// Generated macro for isa_builder (function)
macro_rules! Depcrate_isa_riscv64isa_builder {
() => {
// Module: crate::isa::riscv64
// Provides: {"isa_builder"}
// Dependencies: {}
# [doc = " Create a new `isa::Builder`."] pub fn isa_builder (triple : Triple) -> IsaBuilder { match triple . architecture { Architecture :: Riscv64 (..) => { } _ => unreachable ! () , } IsaBuilder { triple , setup : riscv_settings :: builder () , constructor : isa_constructor , } }
};
}
