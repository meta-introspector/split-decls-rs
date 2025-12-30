// Generated macro for macro_7698 (macro)
macro_rules! Depcrate_mut_mutmacro_7698 {
() => {
// Module: crate::mut_mut
// Provides: {"macro_7698"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for instances of `mut mut` references."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is usually just a typo or a misunderstanding of how references work."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = &mut &mut 1;"] # [doc = ""] # [doc = " let mut x = &mut 1;"] # [doc = " let y = &mut x;"] # [doc = ""] # [doc = " fn foo(x: &mut &mut u32) {}"] # [doc = " ```"] # [doc = " Use instead"] # [doc = " ```no_run"] # [doc = " let x = &mut 1;"] # [doc = ""] # [doc = " let mut x = &mut 1;"] # [doc = " let y = &mut *x; // reborrow"] # [doc = ""] # [doc = " fn foo(x: &mut u32) {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MUT_MUT , pedantic , "usage of double mut-refs, e.g., `&mut &mut ...`" }
};
}
