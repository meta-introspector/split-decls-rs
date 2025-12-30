// Generated macro for test_asm_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_asm_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_asm_expand"}
// Dependencies: {}
# [test] fn test_asm_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! asm {() => {}}
#[rustc_builtin_macro]
macro_rules! global_asm {() => {}}
#[rustc_builtin_macro]
macro_rules! naked_asm {() => {}}

global_asm! {
    ""
}

#[unsafe(naked)]
extern "C" fn foo() {
    naked_asm!("");
}

fn main() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! asm {() => {}}
#[rustc_builtin_macro]
macro_rules! global_asm {() => {}}
#[rustc_builtin_macro]
macro_rules! naked_asm {() => {}}

builtin #global_asm ("")

#[unsafe(naked)]
extern "C" fn foo() {
    builtin #naked_asm ("");
}

fn main() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        builtin #asm ("mov {0}, {1}", "add {0}, 5", out (reg)o, in (reg)i, );
    }
}
"##]] ,) ; }
};
}
