// Generated macro for UserFuncName (enum)
macro_rules! Depcrate_ir_extnameUserFuncName {
() => {
// Module: crate::ir::extname
// Provides: {"UserFuncName"}
// Dependencies: {}
# [doc = " An explicit name for a user-defined function, be it defined in code or in CLIF text."] # [doc = ""] # [doc = " This is used both for naming a function (for debugging purposes) and for declaring external"] # [doc = " functions. In the latter case, this becomes an `ExternalName`, which gets embedded in"] # [doc = " relocations later, etc."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum UserFuncName { # [doc = " A user-defined name, with semantics left to the user."] User (UserExternalName) , # [doc = " A name for a test case, mostly intended for Cranelift testing."] Testcase (TestcaseName) , }
};
}
