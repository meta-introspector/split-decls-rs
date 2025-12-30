// Generated macro for other_5147 (other)
macro_rules! Depcrate_authorizationother_5147 {
() => {
// Module: crate::authorization
// Provides: {"other_5147"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Run an executable tool with enhanced privileges after passing"] # [doc = " suitable authorization procedures."] # [doc = ""] # [doc = ""] # [doc = " Parameter `authorization`: An authorization reference that is used to authorize"] # [doc = " access to the enhanced privileges. It is also passed to the tool for"] # [doc = " further access control."] # [doc = ""] # [doc = " Parameter `pathToTool`: Full pathname to the tool that should be executed"] # [doc = " with enhanced privileges."] # [doc = ""] # [doc = " Parameter `options`: Option bits (reserved). Must be zero."] # [doc = ""] # [doc = " Parameter `arguments`: An argv-style vector of strings to be passed to the tool."] # [doc = ""] # [doc = " Parameter `communicationsPipe`: Assigned a UNIX stdio FILE pointer for"] # [doc = " a bidirectional pipe to communicate with the tool. The tool will have"] # [doc = " this pipe as its standard I/O channels (stdin/stdout). If NULL, do not"] # [doc = " establish a communications pipe."] # [doc = ""] # [doc = ""] # [doc = " This function has been deprecated and should no longer be used."] # [doc = " Use a launchd-launched helper tool and/or the Service Management framework"] # [doc = " for this functionality."] # [deprecated] pub fn AuthorizationExecuteWithPrivileges (authorization : AuthorizationRef , path_to_tool : NonNull < c_char > , options : AuthorizationFlags , arguments : NonNull < AuthorizationString > , communications_pipe : * mut * mut libc :: FILE ,) -> OSStatus ; }
};
}
