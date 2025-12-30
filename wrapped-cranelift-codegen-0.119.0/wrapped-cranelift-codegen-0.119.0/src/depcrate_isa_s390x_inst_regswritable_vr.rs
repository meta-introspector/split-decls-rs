// Generated macro for writable_vr (function)
macro_rules! Depcrate_isa_s390x_inst_regswritable_vr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"writable_vr"}
// Dependencies: {}
# [doc = " Get a writable reference to a VR."] # [allow (dead_code)] pub fn writable_vr (num : u8) -> Writable < Reg > { Writable :: from_reg (vr (num)) }
};
}
