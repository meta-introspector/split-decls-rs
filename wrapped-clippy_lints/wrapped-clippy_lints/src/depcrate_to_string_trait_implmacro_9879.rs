// Generated macro for macro_9879 (macro)
macro_rules! Depcrate_to_string_trait_implmacro_9879 {
() => {
// Module: crate::to_string_trait_impl
// Provides: {"macro_9879"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for direct implementations of `ToString`."] # [doc = " ### Why is this bad?"] # [doc = " This trait is automatically implemented for any type which implements the `Display` trait."] # [doc = " As such, `ToString` shouldn’t be implemented directly: `Display` should be implemented instead,"] # [doc = " and you get the `ToString` implementation for free."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Point {"] # [doc = "   x: usize,"] # [doc = "   y: usize,"] # [doc = " }"] # [doc = ""] # [doc = " impl ToString for Point {"] # [doc = "   fn to_string(&self) -> String {"] # [doc = "     format!(\"({}, {})\", self.x, self.y)"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct Point {"] # [doc = "   x: usize,"] # [doc = "   y: usize,"] # [doc = " }"] # [doc = ""] # [doc = " impl std::fmt::Display for Point {"] # [doc = "   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"] # [doc = "     write!(f, \"({}, {})\", self.x, self.y)"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub TO_STRING_TRAIT_IMPL , style , "check for direct implementations of `ToString`" }
};
}
