// Generated macro for writable_gpr (function)
macro_rules! Depcrate_isa_s390x_inst_regswritable_gpr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"writable_gpr"}
// Dependencies: {}
# [doc = " Get a writable reference to a GPR."] pub fn writable_gpr (num : u8) -> Writable < Reg > { Writable :: from_reg (gpr (num)) }
};
}
