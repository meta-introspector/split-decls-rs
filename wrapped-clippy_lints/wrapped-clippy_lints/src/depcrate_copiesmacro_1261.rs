// Generated macro for macro_1261 (macro)
macro_rules! Depcrate_copiesmacro_1261 {
() => {
// Module: crate::copies
// Provides: {"macro_1261"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for consecutive `if`s with the same function call."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a copy & paste error."] # [doc = " Despite the fact that function can have side effects and `if` works as"] # [doc = " intended, such an approach is implicit and can be considered a \"code smell\"."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " if foo() == bar {"] # [doc = "     …"] # [doc = " } else if foo() == bar {"] # [doc = "     …"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This probably should be:"] # [doc = " ```ignore"] # [doc = " if foo() == bar {"] # [doc = "     …"] # [doc = " } else if foo() == baz {"] # [doc = "     …"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " or if the original code was not a typo and called function mutates a state,"] # [doc = " consider move the mutation out of the `if` condition to avoid similarity to"] # [doc = " a copy & paste error:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let first = foo();"] # [doc = " if first == bar {"] # [doc = "     …"] # [doc = " } else {"] # [doc = "     let second = foo();"] # [doc = "     if second == bar {"] # [doc = "     …"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.41.0"] pub SAME_FUNCTIONS_IN_IF_CONDITION , pedantic , "consecutive `if`s with the same function call" }
};
}
