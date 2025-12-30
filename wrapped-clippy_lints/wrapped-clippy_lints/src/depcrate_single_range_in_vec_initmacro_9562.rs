// Generated macro for macro_9562 (macro)
macro_rules! Depcrate_single_range_in_vec_initmacro_9562 {
() => {
// Module: crate::single_range_in_vec_init
// Provides: {"macro_9562"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `Vec` or array initializations that contain only one range."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is almost always incorrect, as it will result in a `Vec` that has only one element."] # [doc = " Almost always, the programmer intended for it to include all elements in the range or for"] # [doc = " the end of the range to be the length instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = [0..200];"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " // If it was intended to include every element in the range..."] # [doc = " let x = (0..200).collect::<Vec<i32>>();"] # [doc = " // ...Or if 200 was meant to be the len"] # [doc = " let x = [0; 200];"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub SINGLE_RANGE_IN_VEC_INIT , suspicious , "checks for initialization of `Vec` or arrays which consist of a single range" }
};
}
