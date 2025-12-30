// Generated macro for macro_1097 (macro)
macro_rules! Depcrate_castsmacro_1097 {
() => {
// Module: crate::casts
// Provides: {"macro_1097"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the result of a `&self`-taking `as_ptr` being cast to a mutable pointer."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since `as_ptr` takes a `&self`, the pointer won't have write permissions unless interior"] # [doc = " mutability is used, making it unlikely that having it as a mutable pointer is correct."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut vec = Vec::<u8>::with_capacity(1);"] # [doc = " let ptr = vec.as_ptr() as *mut u8;"] # [doc = " unsafe { ptr.write(4) }; // UNDEFINED BEHAVIOUR"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut vec = Vec::<u8>::with_capacity(1);"] # [doc = " let ptr = vec.as_mut_ptr();"] # [doc = " unsafe { ptr.write(4) };"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub AS_PTR_CAST_MUT , nursery , "casting the result of the `&self`-taking `as_ptr` to a mutable pointer" }
};
}
