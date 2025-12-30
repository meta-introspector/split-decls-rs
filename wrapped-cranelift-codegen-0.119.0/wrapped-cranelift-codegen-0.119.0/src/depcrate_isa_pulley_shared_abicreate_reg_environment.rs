// Generated macro for create_reg_environment (function)
macro_rules! Depcrate_isa_pulley_shared_abicreate_reg_environment {
() => {
// Module: crate::isa::pulley_shared::abi
// Provides: {"create_reg_environment"}
// Dependencies: {}
fn create_reg_environment () -> MachineEnv { let preferred_regs_by_class : [Vec < PReg > ; 3] = { let x_registers : Vec < PReg > = (0 .. 16) . map (| x | px_reg (x)) . collect () ; let f_registers : Vec < PReg > = (0 .. 16) . map (| x | pf_reg (x)) . collect () ; let v_registers : Vec < PReg > = (0 .. 32) . map (| x | pv_reg (x)) . collect () ; [x_registers , f_registers , v_registers] } ; let non_preferred_regs_by_class : [Vec < PReg > ; 3] = { let x_registers : Vec < PReg > = (16 .. XReg :: SPECIAL_START) . map (| x | px_reg (x . into ())) . collect () ; let f_registers : Vec < PReg > = (16 .. 32) . map (| x | pf_reg (x)) . collect () ; let v_registers : Vec < PReg > = vec ! [] ; [x_registers , f_registers , v_registers] } ; MachineEnv { preferred_regs_by_class , non_preferred_regs_by_class , fixed_stack_slots : vec ! [] , scratch_by_class : [None , None , None] , } }
};
}
