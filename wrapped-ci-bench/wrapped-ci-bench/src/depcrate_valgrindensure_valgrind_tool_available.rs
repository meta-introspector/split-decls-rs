// Generated macro for ensure_valgrind_tool_available (function)
macro_rules! Depcrate_valgrindensure_valgrind_tool_available {
() => {
// Module: crate::valgrind
// Provides: {"ensure_valgrind_tool_available"}
// Dependencies: {}
# [doc = " Returns an error if valgrind is not available"] fn ensure_valgrind_tool_available (tool : & str) -> anyhow :: Result < () > { let result = Command :: new ("valgrind") . arg (tool) . arg ("--version") . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () ; match result { Err (e) => anyhow :: bail ! ("Unexpected error while launching valgrind {tool}. Error: {}" , e) , Ok (status) => { if status . success () { Ok (()) } else { anyhow :: bail ! ("Failed to launch valgrind {tool}. Error: {}. Please ensure that valgrind is installed and on the $PATH." , status) } } } }
};
}
