// Generated macro for criterion_group_bench (macro)
macro_rules! Depcratecriterion_group_bench {
() => {
// Module: crate
// Provides: {"criterion_group_bench"}
// Dependencies: {}
# [macro_export] macro_rules ! criterion_group_bench { ($ Name : ident , $ Target : ident) => { # [cfg (any (target_arch = "x86_64" , target_arch = "x86" , all (target_arch = "aarch64" , target_os = "linux")))] criterion_group ! (name = $ Name ; config = Criterion :: default () . with_measurement (criterion_cycles_per_byte :: CyclesPerByte) ; targets = $ Target) ; # [cfg (not (any (target_arch = "x86_64" , target_arch = "x86" , all (target_arch = "aarch64" , target_os = "linux"))))] criterion_group ! (name = $ Name ; config = Criterion :: default () ; targets = $ Target) ; } }
};
}
