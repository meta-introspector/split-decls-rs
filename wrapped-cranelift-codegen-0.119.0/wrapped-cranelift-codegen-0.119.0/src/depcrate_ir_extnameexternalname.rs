// Generated macro for ExternalName (enum)
macro_rules! Depcrate_ir_extnameExternalName {
() => {
// Module: crate::ir::extname
// Provides: {"ExternalName"}
// Dependencies: {}
# [doc = " The name of an external is either a reference to a user-defined symbol"] # [doc = " table, or a short sequence of ascii bytes so that test cases do not have"] # [doc = " to keep track of a symbol table."] # [doc = ""] # [doc = " External names are primarily used as keys by code using Cranelift to map"] # [doc = " from a `cranelift_codegen::ir::FuncRef` or similar to additional associated"] # [doc = " data."] # [doc = ""] # [doc = " External names can also serve as a primitive testing and debugging tool."] # [doc = " In particular, many `.clif` test files use function names to identify"] # [doc = " functions."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum ExternalName { # [doc = " A reference to a name in a user-defined symbol table."] User (UserExternalNameRef) , # [doc = " A test case function name of up to a hardcoded amount of ascii"] # [doc = " characters. This is not intended to be used outside test cases."] TestCase (TestcaseName) , # [doc = " A well-known runtime library function."] LibCall (LibCall) , # [doc = " A well-known symbol."] KnownSymbol (KnownSymbol) , }
};
}
