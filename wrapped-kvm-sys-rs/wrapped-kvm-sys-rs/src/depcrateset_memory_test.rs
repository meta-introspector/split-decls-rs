// Generated macro for set_memory_test (function)
macro_rules! Depcrateset_memory_test {
() => {
// Module: crate
// Provides: {"set_memory_test"}
// Dependencies: {}
# [test] fn set_memory_test () { let mut anon_mmap = Mmap :: anonymous (16 * (1 << 12) , Protection :: ReadWrite) . unwrap () ; let slice = unsafe { anon_mmap . as_mut_slice () } ; let h = System :: initialize () . unwrap () ; let mut vm = VirtualMachine :: create (& h) . unwrap () ; vm . set_user_memory_region (0 , slice , 0) . unwrap () ; }
};
}
