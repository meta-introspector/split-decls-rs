// Generated macro for WildcardArm (struct)
macro_rules! Depcrate_match_tokenWildcardArm {
() => {
// Module: crate::match_token
// Provides: {"WildcardArm"}
// Dependencies: {}
# [doc = " Description of a wildcard match arm."] # [doc = ""] # [doc = " We defer generating code for these until we process the last, catch-all"] # [doc = " arm.  This isn't part of the AST produced by `parse()`; it's created"] # [doc = " while processing that AST."] struct WildcardArm { binding : Tokens , kind : TagKind , expr : P < ast :: Expr > , }
};
}
