// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
std :: arch :: global_asm ! (r#"
    .text
    .global rust_plus_one_global_asm
    .type rust_plus_one_global_asm, @function
rust_plus_one_global_asm:
    movl (%rdi), %eax
    inc %eax
    retq
"# , options (att_syntax)) ;
};
}
