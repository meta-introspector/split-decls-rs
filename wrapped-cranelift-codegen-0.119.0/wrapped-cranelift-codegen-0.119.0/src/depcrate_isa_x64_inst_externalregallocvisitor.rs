// Generated macro for RegallocVisitor (struct)
macro_rules! Depcrate_isa_x64_inst_externalRegallocVisitor {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"RegallocVisitor"}
// Dependencies: {}
# [doc = " A wrapper to implement the `cranelift-assembler-x64` register allocation trait,"] # [doc = " `RegallocVisitor`, in terms of the trait used in Cranelift,"] # [doc = " `OperandVisitor`."] pub (crate) struct RegallocVisitor < 'a , T > where T : OperandVisitorImpl , { pub collector : & 'a mut T , }
};
}
