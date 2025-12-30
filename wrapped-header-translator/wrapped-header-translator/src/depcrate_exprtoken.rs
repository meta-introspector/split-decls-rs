// Generated macro for Token (enum)
macro_rules! Depcrate_exprToken {
() => {
// Module: crate::expr
// Provides: {"Token"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] # [allow (clippy :: upper_case_acronyms)] # [allow (clippy :: large_enum_variant)] pub enum Token { Punctuation (String) , Literal (String) , UnknownIdent (String) , ByteChar (u8) , FourChar (FourCharCode) , CStr (String) , CFStringBegin , NSStringBegin , CFUUID (String) , Expr (Expr) , Cast { to : String } , }
};
}
