// Generated macro for OperatorName (enum)
macro_rules! Depcrate_astOperatorName {
() => {
// Module: crate::ast
// Provides: {"OperatorName"}
// Dependencies: {}
# [doc = " The `<operator-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <operator-name> ::= <simple-operator-name>"] # [doc = "                 ::= cv <type>               # (cast)"] # [doc = "                 ::= li <source-name>        # operator \"\""] # [doc = "                 ::= v <digit> <source-name> # vendor extended operator"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum OperatorName { # [doc = " A simple operator name."] Simple (SimpleOperatorName) , # [doc = " A type cast."] Cast (TypeHandle) , # [doc = " A type conversion."] Conversion (TypeHandle) , # [doc = " Operator literal, ie `operator \"\"`."] Literal (SourceName) , # [doc = " A non-standard, vendor extension operator."] VendorExtension (u8 , SourceName) , }
};
}
