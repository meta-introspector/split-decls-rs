// Generated macro for macro_2575 (macro)
macro_rules! Depcrate_functionsmacro_2575 {
() => {
// Module: crate::functions
// Provides: {"macro_2575"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for public functions that have no"] # [doc = " `#[must_use]` attribute, but return something not already marked"] # [doc = " must-use, have no mutable arg and mutate no statics."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Not bad at all, this lint just shows places where"] # [doc = " you could add the attribute."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint only checks the arguments for mutable"] # [doc = " types without looking if they are actually changed. On the other hand,"] # [doc = " it also ignores a broad range of potentially interesting side effects,"] # [doc = " because we cannot decide whether the programmer intends the function to"] # [doc = " be called for the side effect or the result. Expect many false"] # [doc = " positives. At least we don't lint if the result type is unit or already"] # [doc = " `#[must_use]`."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " // this could be annotated with `#[must_use]`."] # [doc = " pub fn id<T>(t: T) -> T { t }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub MUST_USE_CANDIDATE , pedantic , "function or method that could take a `#[must_use]` attribute" }
};
}
