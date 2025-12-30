// Generated macro for make_test_flags (function)
macro_rules! Depcrate_isa_riscv64_inst_emit_testsmake_test_flags {
() => {
// Module: crate::isa::riscv64::inst::emit_tests
// Provides: {"make_test_flags"}
// Dependencies: {}
fn make_test_flags () -> (settings :: Flags , super :: super :: riscv_settings :: Flags) { let b = settings :: builder () ; let flags = settings :: Flags :: new (b . clone ()) ; let b2 = super :: super :: riscv_settings :: builder () ; let isa_flags = super :: super :: riscv_settings :: Flags :: new (& flags , & b2) ; (flags , isa_flags) }
};
}
