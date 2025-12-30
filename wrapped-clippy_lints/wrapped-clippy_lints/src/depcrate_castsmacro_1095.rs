// Generated macro for macro_1095 (macro)
macro_rules! Depcrate_castsmacro_1095 {
() => {
// Module: crate::casts
// Provides: {"macro_1095"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of `&expr as *const T` or"] # [doc = " `&mut expr as *mut T`, and suggest using `&raw const` or"] # [doc = " `&raw mut` instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This would improve readability and avoid creating a reference"] # [doc = " that points to an uninitialized value or unaligned place."] # [doc = " Read the `&raw` explanation in the Reference for more information."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let val = 1;"] # [doc = " let p = &val as *const i32;"] # [doc = ""] # [doc = " let mut val_mut = 1;"] # [doc = " let p_mut = &mut val_mut as *mut i32;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let val = 1;"] # [doc = " let p = &raw const val;"] # [doc = ""] # [doc = " let mut val_mut = 1;"] # [doc = " let p_mut = &raw mut val_mut;"] # [doc = " ```"] # [clippy :: version = "1.60.0"] pub BORROW_AS_PTR , pedantic , "borrowing just to cast to a raw pointer" }
};
}
