// Generated macro for agent (function)
macro_rules! Depcrate_utilagent {
() => {
// Module: crate::util
// Provides: {"agent"}
// Dependencies: {}
# [doc = " The name of the `git` client in a format suitable for presentation to a `git` server, using `name` as user-defined portion of the value."] pub fn agent (name : impl Into < String >) -> String { let mut name = name . into () ; if ! name . starts_with ("git/") { name . insert_str (0 , "git/") ; } name }
};
}
