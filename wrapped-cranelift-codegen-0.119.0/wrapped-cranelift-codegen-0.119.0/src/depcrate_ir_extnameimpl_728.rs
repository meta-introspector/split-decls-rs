// Generated macro for impl_728 (impl)
macro_rules! Depcrate_ir_extnameimpl_728 {
() => {
// Module: crate::ir::extname
// Provides: {"impl_728"}
// Dependencies: {}
impl UserFuncName { # [doc = " Creates a new external name from a sequence of bytes. Caller is expected"] # [doc = " to guarantee bytes are only ascii alphanumeric or `_`."] pub fn testcase < T : AsRef < [u8] > > (v : T) -> Self { Self :: Testcase (TestcaseName :: new (v)) } # [doc = " Create a new external name from a user-defined external function reference."] pub fn user (namespace : u32 , index : u32) -> Self { Self :: User (UserExternalName :: new (namespace , index)) } # [doc = " Get a `UserExternalName` if this is a user-defined name."] pub fn get_user (& self) -> Option < & UserExternalName > { match self { UserFuncName :: User (user) => Some (user) , UserFuncName :: Testcase (_) => None , } } }
};
}
