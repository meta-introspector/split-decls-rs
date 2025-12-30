// Generated macro for macro_2953 (macro)
macro_rules! Depcrate_inherent_to_stringmacro_2953 {
() => {
// Module: crate::inherent_to_string
// Provides: {"macro_2953"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the definition of inherent methods with a signature of `to_string(&self) -> String` and if the type implementing this method also implements the `Display` trait."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This method is also implicitly defined if a type implements the `Display` trait. The less versatile inherent method will then shadow the implementation introduced by `Display`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::fmt;"] # [doc = ""] # [doc = " pub struct A;"] # [doc = ""] # [doc = " impl A {"] # [doc = "     pub fn to_string(&self) -> String {"] # [doc = "         \"I am A\".to_string()"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl fmt::Display for A {"] # [doc = "     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"] # [doc = "         write!(f, \"I am A, too\")"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::fmt;"] # [doc = ""] # [doc = " pub struct A;"] # [doc = ""] # [doc = " impl fmt::Display for A {"] # [doc = "     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"] # [doc = "         write!(f, \"I am A\")"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.38.0"] pub INHERENT_TO_STRING_SHADOW_DISPLAY , correctness , "type implements inherent method `to_string()`, which gets shadowed by the implementation of the `Display` trait" }
};
}
