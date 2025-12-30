// Generated macro for isa_constructor (function)
macro_rules! Depcrate_isa_riscv64isa_constructor {
() => {
// Module: crate::isa::riscv64
// Provides: {"isa_constructor"}
// Dependencies: {}
fn isa_constructor (triple : Triple , shared_flags : Flags , builder : & shared_settings :: Builder ,) -> CodegenResult < OwnedTargetIsa > { let isa_flags = riscv_settings :: Flags :: new (& shared_flags , builder) ; if ! isa_flags . has_g () { return Err (CodegenError :: Unsupported ("The RISC-V Backend currently requires all the features in the G Extension enabled" . into () ,)) ; } let backend = Riscv64Backend :: new_with_flags (triple , shared_flags , isa_flags) ; Ok (backend . wrapped ()) }
};
}
