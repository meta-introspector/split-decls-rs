// Generated macro for isa_builder (function)
macro_rules! Depcrate_isa_pulley_sharedisa_builder {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"isa_builder"}
// Dependencies: {}
# [doc = " Create a new Pulley ISA builder."] pub fn isa_builder (triple : Triple) -> IsaBuilder { let constructor = match triple . architecture { Architecture :: Pulley32 | Architecture :: Pulley32be => isa_constructor_32 , Architecture :: Pulley64 | Architecture :: Pulley64be => isa_constructor_64 , other => panic ! ("unexpected architecture {other:?}") , } ; IsaBuilder { triple , setup : self :: settings :: builder () , constructor , } }
};
}
