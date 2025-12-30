// Generated macro for create_vcpu_test (function)
macro_rules! Depcratecreate_vcpu_test {
() => {
// Module: crate
// Provides: {"create_vcpu_test"}
// Dependencies: {}
# [test] fn create_vcpu_test () { let h = System :: initialize () . unwrap () ; let mut vm = VirtualMachine :: create (& h) . unwrap () ; Vcpu :: create (& mut vm) . unwrap () ; }
};
}
