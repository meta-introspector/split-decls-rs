// Generated macro for impl_740 (impl)
macro_rules! Depcrate_ir_extnameimpl_740 {
() => {
// Module: crate::ir::extname
// Provides: {"impl_740"}
// Dependencies: {}
impl ExternalName { # [doc = " Creates a new external name from a sequence of bytes. Caller is expected"] # [doc = " to guarantee bytes are only ascii alphanumeric or `_`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use cranelift_codegen::ir::ExternalName;"] # [doc = " // Create `ExternalName` from a string."] # [doc = " let name = ExternalName::testcase(\"hello\");"] # [doc = " assert_eq!(name.display(None).to_string(), \"%hello\");"] # [doc = " ```"] pub fn testcase < T : AsRef < [u8] > > (v : T) -> Self { Self :: TestCase (TestcaseName :: new (v)) } # [doc = " Create a new external name from a user-defined external function reference."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust"] # [doc = " # use cranelift_codegen::ir::{ExternalName, UserExternalNameRef};"] # [doc = " let user_func_ref: UserExternalNameRef = Default::default(); // usually obtained with `Function::declare_imported_user_function()`"] # [doc = " let name = ExternalName::user(user_func_ref);"] # [doc = " assert_eq!(name.display(None).to_string(), \"userextname0\");"] # [doc = " ```"] pub fn user (func_ref : UserExternalNameRef) -> Self { Self :: User (func_ref) } # [doc = " Returns a display for the current `ExternalName`, with extra context to prettify the"] # [doc = " output."] pub fn display < 'a > (& 'a self , params : Option < & 'a FunctionParameters > ,) -> DisplayableExternalName < 'a > { DisplayableExternalName { name : self , params } } }
};
}
