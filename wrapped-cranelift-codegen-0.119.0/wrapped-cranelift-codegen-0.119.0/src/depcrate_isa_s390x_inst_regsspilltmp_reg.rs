// Generated macro for spilltmp_reg (function)
macro_rules! Depcrate_isa_s390x_inst_regsspilltmp_reg {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"spilltmp_reg"}
// Dependencies: {}
# [doc = " Get a reference to the first temporary, sometimes \"spill temporary\", register. This register is"] # [doc = " used to compute the address of a spill slot when a direct offset addressing mode from FP is not"] # [doc = " sufficient (+/- 2^11 words). We exclude this register from regalloc and reserve it for this"] # [doc = " purpose for simplicity; otherwise we need a multi-stage analysis where we first determine how"] # [doc = " many spill slots we have, then perhaps remove the reg from the pool and recompute regalloc."] # [doc = ""] # [doc = " We use r1 for this because it's a scratch register but is slightly special (used for linker"] # [doc = " veneers). We're free to use it as long as we don't expect it to live through call instructions."] pub fn spilltmp_reg () -> Reg { gpr (1) }
};
}
