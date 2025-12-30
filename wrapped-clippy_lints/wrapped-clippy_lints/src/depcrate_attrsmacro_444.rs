// Generated macro for macro_444 (macro)
macro_rules! Depcrate_attrsmacro_444 {
() => {
// Module: crate::attrs
// Provides: {"macro_444"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for items annotated with `#[inline(always)]`,"] # [doc = " unless the annotated function is empty or simply panics."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " While there are valid uses of this annotation (and once"] # [doc = " you know when to use it, by all means `allow` this lint), it's a common"] # [doc = " newbie-mistake to pepper one's code with it."] # [doc = ""] # [doc = " As a rule of thumb, before slapping `#[inline(always)]` on a function,"] # [doc = " measure if that additional function call really affects your runtime profile"] # [doc = " sufficiently to make up for the increase in compile time."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " False positives, big time. This lint is meant to be"] # [doc = " deactivated by everyone doing serious performance work. This means having"] # [doc = " done the measurement."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " #[inline(always)]"] # [doc = " fn not_quite_hot_code(..) { ... }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INLINE_ALWAYS , pedantic , "use of `#[inline(always)]`" }
};
}
