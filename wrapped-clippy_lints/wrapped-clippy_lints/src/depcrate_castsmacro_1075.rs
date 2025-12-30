// Generated macro for macro_1075 (macro)
macro_rules! Depcrate_castsmacro_1075 {
() => {
// Module: crate::casts
// Provides: {"macro_1075"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `as` casts between raw pointers that don't change their"] # [doc = " constness, namely `*const T` to `*const U` and `*mut T` to `*mut U`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Though `as` casts between raw pointers are not terrible,"] # [doc = " `pointer::cast` is safer because it cannot accidentally change the"] # [doc = " pointer's mutability, nor cast the pointer to other types like `usize`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let ptr: *const u32 = &42_u32;"] # [doc = " let mut_ptr: *mut u32 = &mut 42_u32;"] # [doc = " let _ = ptr as *const i32;"] # [doc = " let _ = mut_ptr as *mut i32;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let ptr: *const u32 = &42_u32;"] # [doc = " let mut_ptr: *mut u32 = &mut 42_u32;"] # [doc = " let _ = ptr.cast::<i32>();"] # [doc = " let _ = mut_ptr.cast::<i32>();"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub PTR_AS_PTR , pedantic , "casting using `as` between raw pointers that doesn't change their constness, where `pointer::cast` could take the place of `as`" }
};
}
