// Generated macro for macro_2614 (macro)
macro_rules! Depcrate_functionsmacro_2614 {
() => {
// Module: crate::functions
// Provides: {"macro_2614"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for getter methods that return a field that doesn't correspond"] # [doc = " to the name of the method, when there is a field's whose name matches that of the method."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is most likely that such a method is a bug caused by a typo or by copy-pasting."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct A {"] # [doc = "     a: String,"] # [doc = "     b: String,"] # [doc = " }"] # [doc = ""] # [doc = " impl A {"] # [doc = "     fn a(&self) -> &str{"] # [doc = "         &self.b"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct A {"] # [doc = "     a: String,"] # [doc = "     b: String,"] # [doc = " }"] # [doc = ""] # [doc = " impl A {"] # [doc = "     fn a(&self) -> &str{"] # [doc = "         &self.a"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub MISNAMED_GETTERS , suspicious , "getter method returning the wrong field" }
};
}
