// Generated macro for macro_1102 (macro)
macro_rules! Depcrate_castsmacro_1102 {
() => {
// Module: crate::casts
// Provides: {"macro_1102"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts of small constant literals or `mem::align_of` results to raw pointers."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This creates a dangling pointer and is better expressed as"] # [doc = " {`std`, `core`}`::ptr::`{`dangling`, `dangling_mut`}."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let ptr = 4 as *const u32;"] # [doc = " let aligned = std::mem::align_of::<u32>() as *const u32;"] # [doc = " let mut_ptr: *mut i64 = 8 as *mut _;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let ptr = std::ptr::dangling::<u32>();"] # [doc = " let aligned = std::ptr::dangling::<u32>();"] # [doc = " let mut_ptr: *mut i64 = std::ptr::dangling_mut();"] # [doc = " ```"] # [clippy :: version = "1.88.0"] pub MANUAL_DANGLING_PTR , style , "casting small constant literals to pointers to create dangling pointers" }
};
}
