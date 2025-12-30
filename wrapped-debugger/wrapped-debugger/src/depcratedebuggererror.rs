// Generated macro for DebuggerError (enum)
macro_rules! DepcrateDebuggerError {
() => {
// Module: crate
// Provides: {"DebuggerError"}
// Dependencies: {}
# [doc = " Possible errors that can occur in the debugger context."] # [derive (Debug , thiserror :: Error)] pub enum DebuggerError { # [doc = " Errors from opening files etc."] # [error ("I/O error: {0}")] Io (# [from] io :: Error) , # [doc = " When a filename can't be extracted from a grammar path."] # [error ("Missing filename")] MissingFilename , # [doc = " Running a debugger requires a grammar to be provided."] # [error ("Open grammar first")] GrammarNotOpened , # [doc = " Running a debugger requires a parsing input to be provided."] # [error ("Open input first")] InputNotOpened , # [doc = " Continuing a debugger session requires starting a session by running a rule."] # [error ("Run rule first")] RunRuleFirst , # [doc = " Parsing finished (i.e. cannot continue the session)."] # [error ("End-of-input reached")] EofReached , # [doc = " Can't create a `Position` in a given input."] # [error ("Invalid position: {0}")] InvalidPosition (usize) , # [doc = " The provided grammar is invalid."] # [doc = " The first element contains a formatted error message."] # [doc = " The second element (`Vec`) contains the errors."] # [error ("Grammar error: {0}")] IncorrectGrammar (String , Vec < Error < Rule > >) , # [doc = " When restarting a session, the previous session"] # [doc = " seem to have panicked."] # [error ("Previous parsing execution panic: {0}")] PreviousRunPanic (String) , }
};
}
