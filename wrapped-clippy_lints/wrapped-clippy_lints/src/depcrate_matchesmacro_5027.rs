// Generated macro for macro_5027 (macro)
macro_rules! Depcrate_matchesmacro_5027 {
() => {
// Module: crate::matches
// Provides: {"macro_5027"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for matches being used to destructure a single-variant enum"] # [doc = " or tuple struct where a `let` will suffice."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Just readability – `let` doesn't nest, whereas a `match` does."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum Wrapper {"] # [doc = "     Data(i32),"] # [doc = " }"] # [doc = ""] # [doc = " let wrapper = Wrapper::Data(42);"] # [doc = ""] # [doc = " let data = match wrapper {"] # [doc = "     Wrapper::Data(i) => i,"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " enum Wrapper {"] # [doc = "     Data(i32),"] # [doc = " }"] # [doc = ""] # [doc = " let wrapper = Wrapper::Data(42);"] # [doc = " let Wrapper::Data(data) = wrapper;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INFALLIBLE_DESTRUCTURING_MATCH , style , "a `match` statement with a single infallible arm instead of a `let`" }
};
}
