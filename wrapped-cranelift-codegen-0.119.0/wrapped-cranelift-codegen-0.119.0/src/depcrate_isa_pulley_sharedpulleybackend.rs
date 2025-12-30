// Generated macro for PulleyBackend (struct)
macro_rules! Depcrate_isa_pulley_sharedPulleyBackend {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"PulleyBackend"}
// Dependencies: {}
# [doc = " A Pulley backend."] pub struct PulleyBackend < P > where P : PulleyTargetKind , { pulley_target : PhantomData < P > , triple : Triple , flags : Flags , isa_flags : PulleyFlags , }
};
}
