// Generated macro for test_trap_encoding (function)
macro_rules! Depcrate_isa_pulley_shared_insttest_trap_encoding {
() => {
// Module: crate::isa::pulley_shared::inst
// Provides: {"test_trap_encoding"}
// Dependencies: {}
# [test] fn test_trap_encoding () { let mut dst = std :: vec :: Vec :: new () ; pulley_interpreter :: encode :: trap (& mut dst) ; assert_eq ! (dst , TRAP_OPCODE) ; }
};
}
