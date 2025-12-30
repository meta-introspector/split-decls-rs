// Generated macro for macro_186 (macro)
macro_rules! Depcrate_astmacro_186 {
() => {
// Module: crate::ast
// Provides: {"macro_186"}
// Dependencies: {}
define_vocabulary ! { # [doc = " A <ref-qualifier> production."] # [doc = ""] # [doc = " ```text"] # [doc = " <ref-qualifier> ::= R   # & ref-qualifier"] # [doc = "                 ::= O   # && ref-qualifier"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum RefQualifier { LValueRef (b"R" , "&") , RValueRef (b"O" , "&&") } }
};
}
