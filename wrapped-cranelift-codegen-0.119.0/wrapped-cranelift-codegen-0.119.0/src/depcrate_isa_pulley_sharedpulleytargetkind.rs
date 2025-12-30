// Generated macro for PulleyTargetKind (trait)
macro_rules! Depcrate_isa_pulley_sharedPulleyTargetKind {
() => {
// Module: crate::isa::pulley_shared
// Provides: {"PulleyTargetKind"}
// Dependencies: {}
# [doc = " A trait to abstract over the different kinds of Pulley targets that exist"] # [doc = " (32- vs 64-bit)."] pub trait PulleyTargetKind : 'static + Clone + Debug + Default + Send + Sync { fn pointer_width () -> PointerWidth ; fn name () -> & 'static str { match Self :: pointer_width () { PointerWidth :: PointerWidth32 => "pulley32" , PointerWidth :: PointerWidth64 => "pulley64" , } } }
};
}
