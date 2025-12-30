// Generated macro for PairedGpr (struct)
macro_rules! Depcrate_isa_x64_inst_externalPairedGpr {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"PairedGpr"}
// Dependencies: {}
# [doc = " A pair of registers, one for reading and one for writing."] # [doc = ""] # [doc = " Due to how Cranelift's SSA form, we must track the read and write registers"] # [doc = " separately prior to register allocation. Once register allocation is"] # [doc = " complete, we expect the hardware encoding for both `read` and `write` to be"] # [doc = " the same."] # [derive (Clone , Copy , Debug)] pub struct PairedGpr { pub (crate) read : Gpr , pub (crate) write : WritableGpr , }
};
}
