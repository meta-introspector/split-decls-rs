// Generated macro for macro_8879 (macro)
macro_rules! Depcrate_partialeq_to_nonemacro_8879 {
() => {
// Module: crate::partialeq_to_none
// Provides: {"macro_8879"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for binary comparisons to a literal `Option::None`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " A programmer checking if some `foo` is `None` via a comparison `foo == None`"] # [doc = " is usually inspired from other programming languages (e.g. `foo is None`"] # [doc = " in Python)."] # [doc = " Checking if a value of type `Option<T>` is (not) equal to `None` in that"] # [doc = " way relies on `T: PartialEq` to do the comparison, which is unneeded."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(f: Option<u32>) -> &'static str {"] # [doc = "     if f != None { \"yay\" } else { \"nay\" }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(f: Option<u32>) -> &'static str {"] # [doc = "     if f.is_some() { \"yay\" } else { \"nay\" }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub PARTIALEQ_TO_NONE , style , "Binary comparison to `Option<T>::None` relies on `T: PartialEq`, which is unneeded" }
};
}
