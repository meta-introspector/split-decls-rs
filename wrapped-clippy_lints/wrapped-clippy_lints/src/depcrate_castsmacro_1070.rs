// Generated macro for macro_1070 (macro)
macro_rules! Depcrate_castsmacro_1070 {
() => {
// Module: crate::casts
// Provides: {"macro_1070"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts, using `as` or `pointer::cast`, from a"] # [doc = " less strictly aligned pointer to a more strictly aligned pointer."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Dereferencing the resulting pointer may be undefined behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Using [`std::ptr::read_unaligned`](https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html) and [`std::ptr::write_unaligned`](https://doc.rust-lang.org/std/ptr/fn.write_unaligned.html) or"] # [doc = " similar on the resulting pointer is fine. Is over-zealous: casts with"] # [doc = " manual alignment checks or casts like `u64` -> `u8` -> `u16` can be"] # [doc = " fine. Miri is able to do a more in-depth analysis."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = (&1u8 as *const u8) as *const u16;"] # [doc = " let _ = (&mut 1u8 as *mut u8) as *mut u16;"] # [doc = ""] # [doc = " (&1u8 as *const u8).cast::<u16>();"] # [doc = " (&mut 1u8 as *mut u8).cast::<u16>();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CAST_PTR_ALIGNMENT , pedantic , "cast from a pointer to a more strictly aligned pointer" }
};
}
