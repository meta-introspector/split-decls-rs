// Generated macro for Build (enum)
macro_rules! Depcrate_parserBuild {
() => {
// Module: crate::parser
// Provides: {"Build"}
// Dependencies: {}
# [doc = " An ephemeral type for representing the expression stack."] # [doc = ""] # [doc = " Everything on the stack is either a regular expression or a marker"] # [doc = " indicating the opening of a group (possibly non-capturing). The opening"] # [doc = " of a group copies the current flag state, which is reset on the parser"] # [doc = " state once the group closes."] # [derive (Debug)] enum Build { Expr (Expr) , LeftParen { i : CaptureIndex , name : CaptureName , chari : usize , old_flags : Flags , } , }
};
}
