// Generated macro for create_reg_env_systemv (function)
macro_rules! Depcrate_isa_x64_abicreate_reg_env_systemv {
() => {
// Module: crate::isa::x64::abi
// Provides: {"create_reg_env_systemv"}
// Dependencies: {}
fn create_reg_env_systemv (enable_pinned_reg : bool) -> MachineEnv { fn preg (r : Reg) -> PReg { r . to_real_reg () . unwrap () . into () } let mut env = MachineEnv { preferred_regs_by_class : [vec ! [preg (regs :: rsi ()) , preg (regs :: rdi ()) , preg (regs :: rax ()) , preg (regs :: rcx ()) , preg (regs :: rdx ()) , preg (regs :: r8 ()) , preg (regs :: r9 ()) , preg (regs :: r10 ()) , preg (regs :: r11 ()) ,] , vec ! [preg (regs :: xmm0 ()) , preg (regs :: xmm1 ()) , preg (regs :: xmm2 ()) , preg (regs :: xmm3 ()) , preg (regs :: xmm4 ()) , preg (regs :: xmm5 ()) , preg (regs :: xmm6 ()) , preg (regs :: xmm7 ()) ,] , vec ! [] ,] , non_preferred_regs_by_class : [vec ! [preg (regs :: rbx ()) , preg (regs :: r12 ()) , preg (regs :: r13 ()) , preg (regs :: r14 ()) ,] , vec ! [preg (regs :: xmm8 ()) , preg (regs :: xmm9 ()) , preg (regs :: xmm10 ()) , preg (regs :: xmm11 ()) , preg (regs :: xmm12 ()) , preg (regs :: xmm13 ()) , preg (regs :: xmm14 ()) , preg (regs :: xmm15 ()) ,] , vec ! [] ,] , fixed_stack_slots : vec ! [] , scratch_by_class : [None , None , None] , } ; debug_assert_eq ! (regs :: r15 () , regs :: pinned_reg ()) ; if ! enable_pinned_reg { env . non_preferred_regs_by_class [0] . push (preg (regs :: r15 ())) ; } env }
};
}
