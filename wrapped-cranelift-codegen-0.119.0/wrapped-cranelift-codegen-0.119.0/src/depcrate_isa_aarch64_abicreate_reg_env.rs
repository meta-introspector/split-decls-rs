// Generated macro for create_reg_env (function)
macro_rules! Depcrate_isa_aarch64_abicreate_reg_env {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"create_reg_env"}
// Dependencies: {}
fn create_reg_env (enable_pinned_reg : bool) -> MachineEnv { fn preg (r : Reg) -> PReg { r . to_real_reg () . unwrap () . into () } let mut env = MachineEnv { preferred_regs_by_class : [vec ! [preg (xreg (0)) , preg (xreg (1)) , preg (xreg (2)) , preg (xreg (3)) , preg (xreg (4)) , preg (xreg (5)) , preg (xreg (6)) , preg (xreg (7)) , preg (xreg (8)) , preg (xreg (9)) , preg (xreg (10)) , preg (xreg (11)) , preg (xreg (12)) , preg (xreg (13)) , preg (xreg (14)) , preg (xreg (15)) ,] , vec ! [preg (vreg (0)) , preg (vreg (1)) , preg (vreg (2)) , preg (vreg (3)) , preg (vreg (4)) , preg (vreg (5)) , preg (vreg (6)) , preg (vreg (7)) , preg (vreg (16)) , preg (vreg (17)) , preg (vreg (18)) , preg (vreg (19)) , preg (vreg (20)) , preg (vreg (21)) , preg (vreg (22)) , preg (vreg (23)) , preg (vreg (24)) , preg (vreg (25)) , preg (vreg (26)) , preg (vreg (27)) , preg (vreg (28)) , preg (vreg (29)) , preg (vreg (30)) , preg (vreg (31)) ,] , vec ! [] ,] , non_preferred_regs_by_class : [vec ! [preg (xreg (19)) , preg (xreg (20)) , preg (xreg (22)) , preg (xreg (23)) , preg (xreg (24)) , preg (xreg (25)) , preg (xreg (26)) , preg (xreg (27)) , preg (xreg (28)) ,] , vec ! [preg (vreg (8)) , preg (vreg (9)) , preg (vreg (10)) , preg (vreg (11)) , preg (vreg (12)) , preg (vreg (13)) , preg (vreg (14)) , preg (vreg (15)) ,] , vec ! [] ,] , fixed_stack_slots : vec ! [] , scratch_by_class : [None , None , None] , } ; if ! enable_pinned_reg { debug_assert_eq ! (PINNED_REG , 21) ; env . non_preferred_regs_by_class [0] . push (preg (xreg (PINNED_REG))) ; } env }
};
}
