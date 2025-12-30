// Generated macro for Terminator (enum)
macro_rules! DepcrateTerminator {
() => {
// Module: crate
// Provides: {"Terminator"}
// Dependencies: {}
# [doc = " A record terminator."] # [doc = ""] # [doc = " Use this to specify the record terminator while parsing CSV. The default is"] # [doc = " CRLF, which treats `\\r`, `\\n` or `\\r\\n` as a single record terminator."] # [derive (Clone , Copy , Debug , Default)] # [non_exhaustive] pub enum Terminator { # [doc = " Parses `\\r`, `\\n` or `\\r\\n` as a single record terminator."] # [default] CRLF , # [doc = " Parses the byte given as a record terminator."] Any (u8) , }
};
}
