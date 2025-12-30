// Generated macro for macro_7966 (macro)
macro_rules! Depcrate_needless_pass_by_ref_mutmacro_7966 {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"macro_7966"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Check if a `&mut` function argument is actually used mutably."] # [doc = ""] # [doc = " Be careful if the function is publicly reexported as it would break compatibility with"] # [doc = " users of this function, when the users pass this function as an argument."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Less `mut` means less fights with the borrow checker. It can also lead to more"] # [doc = " opportunities for parallelization."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(y: &mut i32) -> i32 {"] # [doc = "     12 + *y"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(y: &i32) -> i32 {"] # [doc = "     12 + *y"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub NEEDLESS_PASS_BY_REF_MUT , nursery , "using a `&mut` argument when it's not mutated" }
};
}
