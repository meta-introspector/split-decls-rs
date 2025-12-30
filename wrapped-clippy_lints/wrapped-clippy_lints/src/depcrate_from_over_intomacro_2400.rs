// Generated macro for macro_2400 (macro)
macro_rules! Depcrate_from_over_intomacro_2400 {
() => {
// Module: crate::from_over_into
// Provides: {"macro_2400"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Searches for implementations of the `Into<..>` trait and suggests to implement `From<..>` instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " According the std docs implementing `From<..>` is preferred since it gives you `Into<..>` for free where the reverse isn't true."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct StringWrapper(String);"] # [doc = ""] # [doc = " impl Into<StringWrapper> for String {"] # [doc = "     fn into(self) -> StringWrapper {"] # [doc = "         StringWrapper(self)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct StringWrapper(String);"] # [doc = ""] # [doc = " impl From<String> for StringWrapper {"] # [doc = "     fn from(s: String) -> StringWrapper {"] # [doc = "         StringWrapper(s)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub FROM_OVER_INTO , style , "Warns on implementations of `Into<..>` to use `From<..>`" }
};
}
