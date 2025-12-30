// Generated macro for impl_2615 (impl)
macro_rules! Depcrate_isa_pulley_shared_instimpl_2615 {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"impl_2615"}
// Dependencies: {}
impl < P > From < Inst > for InstAndKind < P > where P : PulleyTargetKind , { fn from (inst : Inst) -> Self { Self { inst , kind : PhantomData , } } }
};
}
