// Generated macro for PulleyMachineDeps (struct)
macro_rules! Depcrate_isa_pulley_shared_abiPulleyMachineDeps {
() => {
// Module: crate::isa::pulley_shared::abi
// Provides: {"PulleyMachineDeps"}
// Dependencies: {}
# [doc = " Pulley-specific ABI behavior. This struct just serves as an implementation"] # [doc = " point for the trait; it is never actually instantiated."] pub struct PulleyMachineDeps < P > where P : PulleyTargetKind , { _phantom : PhantomData < P > , }
};
}
