// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A test context."] # [non_exhaustive] pub struct Context { # [doc = " The complete module test path  "] pub module : & 'static str , # [doc = " The test function name  "] pub name : & 'static str , # [doc = " The test description if present"] pub description : Option < & 'static str > , # [doc = " The cardinal case number if it's a test case"] pub case : Option < usize > , # [doc = " Start time"] pub start : std :: time :: Instant , }
};
}
