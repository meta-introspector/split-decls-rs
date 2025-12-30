// Generated macro for impl_2616 (impl)
macro_rules! Depcrate_isa_pulley_shared_instimpl_2616 {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"impl_2616"}
// Dependencies: {}
impl < P > From < RawInst > for InstAndKind < P > where P : PulleyTargetKind , { fn from (inst : RawInst) -> Self { Self { inst : inst . into () , kind : PhantomData , } } }
};
}
