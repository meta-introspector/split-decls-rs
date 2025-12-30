// Generated macro for sreg_test (function)
macro_rules! Depcratesreg_test {
() => {
// Module: crate
// Provides: {"sreg_test"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] # [test] fn sreg_test () { let h = System :: initialize () . unwrap () ; let mut vm = VirtualMachine :: create (& h) . unwrap () ; let mut vcpu = Vcpu :: create (& mut vm) . unwrap () ; let mut sregs = vcpu . get_sregs () . unwrap () ; sregs . cr0 = 0x1 ; vcpu . set_sregs (& sregs) . unwrap () ; assert ! (vcpu . get_sregs () . unwrap () . cr0 == 0x1) ; }
};
}
