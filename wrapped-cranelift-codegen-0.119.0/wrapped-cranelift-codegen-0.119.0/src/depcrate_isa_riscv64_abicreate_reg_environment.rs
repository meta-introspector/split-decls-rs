// Generated macro for create_reg_environment (function)
macro_rules! Depcrate_isa_riscv64_abicreate_reg_environment {
() => {
// Module: crate::isa::riscv64::abi
// Provides: {"create_reg_environment"}
// Dependencies: {}
fn create_reg_environment () -> MachineEnv { let preferred_regs_by_class : [Vec < PReg > ; 3] = { let x_registers : Vec < PReg > = (10 ..= 15) . map (px_reg) . collect () ; let f_registers : Vec < PReg > = (10 ..= 15) . map (pf_reg) . collect () ; let v_registers : Vec < PReg > = (8 ..= 15) . map (pv_reg) . collect () ; [x_registers , f_registers , v_registers] } ; let non_preferred_regs_by_class : [Vec < PReg > ; 3] = { let x_registers : Vec < PReg > = (5 ..= 7) . chain (16 ..= 17) . chain (28 ..= 29) . chain (9 ..= 9) . chain (18 ..= 27) . map (px_reg) . collect () ; let f_registers : Vec < PReg > = (0 ..= 7) . chain (16 ..= 17) . chain (28 ..= 31) . chain (8 ..= 9) . chain (18 ..= 27) . map (pf_reg) . collect () ; let v_registers = (0 ..= 7) . chain (16 ..= 31) . map (pv_reg) . collect () ; [x_registers , f_registers , v_registers] } ; MachineEnv { preferred_regs_by_class , non_preferred_regs_by_class , fixed_stack_slots : vec ! [] , scratch_by_class : [None , None , None] , } }
};
}
