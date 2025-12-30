// Generated macro for Tool (struct)
macro_rules! Depcrate_toolTool {
() => {
// Module: crate::tool
// Provides: {"Tool"}
// Dependencies: {}
# [doc = " Configuration used to represent an invocation of a C compiler."] # [doc = ""] # [doc = " This can be used to figure out what compiler is in use, what the arguments"] # [doc = " to it are, and what the environment variables look like for the compiler."] # [doc = " This can be used to further configure other build systems (e.g. forward"] # [doc = " along CC and/or CFLAGS) or the `to_command` method can be used to run the"] # [doc = " compiler itself."] # [derive (Clone , Debug)] # [allow (missing_docs)] pub struct Tool { pub (crate) path : PathBuf , pub (crate) cc_wrapper_path : Option < PathBuf > , pub (crate) cc_wrapper_args : Vec < OsString > , pub (crate) args : Vec < OsString > , pub (crate) env : Vec < (OsString , OsString) > , pub (crate) family : ToolFamily , pub (crate) cuda : bool , pub (crate) removed_args : Vec < OsString > , pub (crate) has_internal_target_arg : bool , }
};
}
