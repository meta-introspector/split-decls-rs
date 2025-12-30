// Generated macro for isa_builder (function)
macro_rules! Depcrate_isa_x64isa_builder {
() => {
// Module: crate::isa::x64
// Provides: {"isa_builder"}
// Dependencies: {}
# [doc = " Create a new `isa::Builder`."] pub (crate) fn isa_builder (triple : Triple) -> IsaBuilder { IsaBuilder { triple , setup : x64_settings :: builder () , constructor : isa_constructor , } }
};
}
