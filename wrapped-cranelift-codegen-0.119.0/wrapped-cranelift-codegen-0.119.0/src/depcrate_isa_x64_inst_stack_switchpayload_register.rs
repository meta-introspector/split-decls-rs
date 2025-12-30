// Generated macro for payload_register (function)
macro_rules! Depcrate_isa_x64_inst_stack_switchpayload_register {
() => {
// Module: crate::isa::x64::inst::stack_switch
// Provides: {"payload_register"}
// Dependencies: {}
# [doc = " The register used for handing over the payload when switching stacks."] # [doc = ""] # [doc = " We must use a fixed register for sending and receiving the payload: When"] # [doc = " switching from one stack to another using two matching``stack_switch``"] # [doc = " instructions, they must agree on the register where the payload is, similar"] # [doc = " to a calling convention. The same holds when `stack_switch`-ing to a newly"] # [doc = " initialized stack, where the entry trampoline must know which register the"] # [doc = " payload is in."] pub fn payload_register () -> Reg { regs :: rdi () }
};
}
