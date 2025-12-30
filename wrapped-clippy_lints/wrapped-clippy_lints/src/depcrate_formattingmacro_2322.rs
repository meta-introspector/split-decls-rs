// Generated macro for macro_2322 (macro)
macro_rules! Depcrate_formattingmacro_2322 {
() => {
// Module: crate::formatting
// Provides: {"macro_2322"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks the formatting of a unary operator on the right hand side"] # [doc = " of a binary operator. It lints if there is no space between the binary and unary operators,"] # [doc = " but there is a space between the unary and its operand."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is either a typo in the binary operator or confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let foo = true;"] # [doc = " # let bar = false;"] # [doc = " // &&! looks like a different operator"] # [doc = " if foo &&! bar {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let foo = true;"] # [doc = " # let bar = false;"] # [doc = " if foo && !bar {}"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub SUSPICIOUS_UNARY_OP_FORMATTING , suspicious , "suspicious formatting of unary `-` or `!` on the RHS of a BinOp" }
};
}
