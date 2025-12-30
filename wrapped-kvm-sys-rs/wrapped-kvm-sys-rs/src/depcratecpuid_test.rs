// Generated macro for cpuid_test (function)
macro_rules! Depcratecpuid_test {
() => {
// Module: crate
// Provides: {"cpuid_test"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] # [test] fn cpuid_test () { let h = System :: initialize () . unwrap () ; if h . check_capability (Capability :: ExtCpuid) == 1 { let mut cpuid = h . get_supported_cpuid () . unwrap () ; let mut vm = VirtualMachine :: create (& h) . unwrap () ; let mut vcpu = Vcpu :: create (& mut vm) . unwrap () ; vcpu . set_cpuid2 (& mut cpuid) . unwrap () ; } }
};
}
