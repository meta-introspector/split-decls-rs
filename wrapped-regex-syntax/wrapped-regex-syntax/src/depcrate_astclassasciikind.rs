// Generated macro for ClassAsciiKind (enum)
macro_rules! Depcrate_astClassAsciiKind {
() => {
// Module: crate::ast
// Provides: {"ClassAsciiKind"}
// Dependencies: {}
# [doc = " The available ASCII character classes."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum ClassAsciiKind { # [doc = " `[0-9A-Za-z]`"] Alnum , # [doc = " `[A-Za-z]`"] Alpha , # [doc = " `[\\x00-\\x7F]`"] Ascii , # [doc = " `[ \\t]`"] Blank , # [doc = " `[\\x00-\\x1F\\x7F]`"] Cntrl , # [doc = " `[0-9]`"] Digit , # [doc = " `[!-~]`"] Graph , # [doc = " `[a-z]`"] Lower , # [doc = " `[ -~]`"] Print , # [doc = " ``[!-/:-@\\[-`{-~]``"] Punct , # [doc = " `[\\t\\n\\v\\f\\r ]`"] Space , # [doc = " `[A-Z]`"] Upper , # [doc = " `[0-9A-Za-z_]`"] Word , # [doc = " `[0-9A-Fa-f]`"] Xdigit , }
};
}
