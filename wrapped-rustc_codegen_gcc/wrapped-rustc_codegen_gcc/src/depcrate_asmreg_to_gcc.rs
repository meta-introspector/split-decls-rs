// Generated macro for reg_to_gcc (function)
macro_rules! Depcrate_asmreg_to_gcc {
() => {
// Module: crate::asm
// Provides: {"reg_to_gcc"}
// Dependencies: {}
# [doc = " Converts a register class to a GCC constraint code."] fn reg_to_gcc (reg_or_reg_class : InlineAsmRegOrRegClass) -> ConstraintOrRegister { match reg_or_reg_class { InlineAsmRegOrRegClass :: Reg (reg) => { ConstraintOrRegister :: Register (explicit_reg_to_gcc (reg)) } InlineAsmRegOrRegClass :: RegClass (reg_class) => { ConstraintOrRegister :: Constraint (reg_class_to_gcc (reg_class)) } } }
};
}
