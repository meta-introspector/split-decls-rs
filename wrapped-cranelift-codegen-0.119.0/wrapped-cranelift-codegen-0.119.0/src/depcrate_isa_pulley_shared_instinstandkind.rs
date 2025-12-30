// Generated macro for InstAndKind (struct)
macro_rules! Depcrate_isa_pulley_shared_instInstAndKind {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"InstAndKind"}
// Dependencies: {}
# [doc = " A newtype over a Pulley instruction that also carries a phantom type"] # [doc = " parameter describing whether we are targeting 32- or 64-bit Pulley bytecode."] # [doc = ""] # [doc = " Implements `Deref`, `DerefMut`, and `From`/`Into` for `Inst` to allow for"] # [doc = " seamless conversion between `Inst` and `InstAndKind`."] # [derive (Clone , Debug)] pub struct InstAndKind < P > where P : PulleyTargetKind , { inst : Inst , kind : PhantomData < P > , }
};
}
