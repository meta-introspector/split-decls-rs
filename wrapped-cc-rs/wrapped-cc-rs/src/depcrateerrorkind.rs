// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Represents the types of errors that may occur while using cc-rs."] # [derive (Clone , Debug)] enum ErrorKind { # [doc = " Error occurred while performing I/O."] IOError , # [doc = " Environment variable not found, with the var in question as extra info."] EnvVarNotFound , # [doc = " Error occurred while using external tools (ie: invocation of compiler)."] ToolExecError , # [doc = " Error occurred due to missing external tools."] ToolNotFound , # [doc = " One of the function arguments failed validation."] InvalidArgument , # [doc = " No known macro is defined for the compiler when discovering tool family."] ToolFamilyMacroNotFound , # [doc = " Invalid target."] InvalidTarget , # [doc = " Unknown target."] UnknownTarget , # [doc = " Invalid rustc flag."] InvalidFlag , # [cfg (feature = "parallel")] # [doc = " jobserver helpthread failure"] JobserverHelpThreadError , # [doc = " `cc` has been disabled by an environment variable."] Disabled , }
};
}
