// Generated macro for ReplSession (struct)
macro_rules! Depcrate_replReplSession {
() => {
// Module: crate::repl
// Provides: {"ReplSession"}
// Dependencies: {}
# [doc = " A repl session: e.g. bash or the python shell:"] # [doc = " you have a prompt where a user inputs commands and the shell"] # [doc = " which executes them and manages IO streams."] # [derive (Debug)] pub struct ReplSession < S > { # [doc = " A pseudo-teletype session with a spawned process."] session : S , # [doc = " The prompt, used for `wait_for_prompt`,"] # [doc = " e.g. \">>> \" for python."] prompt : String , # [doc = " A command which will be called before termination."] quit_command : Option < String > , # [doc = " Flag to see if a echo is turned on."] is_echo_on : bool , }
};
}
