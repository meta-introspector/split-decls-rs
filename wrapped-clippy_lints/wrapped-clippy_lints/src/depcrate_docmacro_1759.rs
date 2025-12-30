// Generated macro for macro_1759 (macro)
macro_rules! Depcrate_docmacro_1759 {
() => {
// Module: crate::doc
// Provides: {"macro_1759"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects doc comment linebreaks that use double spaces to separate lines, instead of back-slash (`\\`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Double spaces, when used as doc comment linebreaks, can be difficult to see, and may"] # [doc = " accidentally be removed during automatic formatting or manual refactoring. The use of a back-slash (`\\`)"] # [doc = " is clearer in this regard."] # [doc = ""] # [doc = " ### Example"] # [doc = " The two replacement dots in this example represent a double space."] # [doc = " ```no_run"] # [doc = " /// This command takes two numbers as inputs and··"] # [doc = " /// adds them together, and then returns the result."] # [doc = " fn add(l: i32, r: i32) -> i32 {"] # [doc = "     l + r"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " /// This command takes two numbers as inputs and\\"] # [doc = " /// adds them together, and then returns the result."] # [doc = " fn add(l: i32, r: i32) -> i32 {"] # [doc = "     l + r"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub DOC_COMMENT_DOUBLE_SPACE_LINEBREAKS , pedantic , "double-space used for doc comment linebreak instead of `\\`" }
};
}
