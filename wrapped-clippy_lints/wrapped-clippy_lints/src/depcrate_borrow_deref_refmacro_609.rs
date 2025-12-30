// Generated macro for macro_609 (macro)
macro_rules! Depcrate_borrow_deref_refmacro_609 {
() => {
// Module: crate::borrow_deref_ref
// Provides: {"macro_609"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `&*(&T)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Dereferencing and then borrowing a reference value has no effect in most cases."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " False negative on such code:"] # [doc = " ```no_run"] # [doc = " let x = &12;"] # [doc = " let addr_x = &x as *const _ as usize;"] # [doc = " let addr_y = &&*x as *const _ as usize; // assert ok now, and lint triggered."] # [doc = "                                         // But if we fix it, assert will fail."] # [doc = " assert_ne!(addr_x, addr_y);"] # [doc = " ```"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let s = &String::new();"] # [doc = ""] # [doc = " let a: &String = &* s;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let s = &String::new();"] # [doc = " let a: &String = s;"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub BORROW_DEREF_REF , complexity , "deref on an immutable reference returns the same type as itself" }
};
}
