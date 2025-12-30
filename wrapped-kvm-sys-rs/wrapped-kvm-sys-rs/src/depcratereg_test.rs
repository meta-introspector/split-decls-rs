// Generated macro for reg_test (function)
macro_rules! Depcratereg_test {
() => {
// Module: crate
// Provides: {"reg_test"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] # [test] fn reg_test () { let h = System :: initialize () . unwrap () ; let mut vm = VirtualMachine :: create (& h) . unwrap () ; let mut vcpu = Vcpu :: create (& mut vm) . unwrap () ; let mut regs = vcpu . get_regs () . unwrap () ; regs . rax = 0x1 ; vcpu . set_regs (& regs) . unwrap () ; assert ! (vcpu . get_regs () . unwrap () . rax == 0x1) ; }
};
}
