// Generated macro for isa_builder (function)
macro_rules! Depcrate_isa_aarch64isa_builder {
() => {
// Module: crate::isa::aarch64
// Provides: {"isa_builder"}
// Dependencies: {}
# [doc = " Create a new `isa::Builder`."] pub fn isa_builder (triple : Triple) -> IsaBuilder { assert ! (triple . architecture == Architecture :: Aarch64 (Aarch64Architecture :: Aarch64)) ; IsaBuilder { triple , setup : aarch64_settings :: builder () , constructor : | triple , shared_flags , builder | { let isa_flags = aarch64_settings :: Flags :: new (& shared_flags , builder) ; let backend = AArch64Backend :: new_with_flags (triple , shared_flags , isa_flags) ; Ok (backend . wrapped ()) } , } }
};
}
