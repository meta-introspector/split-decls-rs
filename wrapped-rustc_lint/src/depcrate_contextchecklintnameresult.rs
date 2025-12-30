// Generated macro for CheckLintNameResult (enum)
macro_rules! Depcrate_contextCheckLintNameResult {
() => {
// Module: crate::context
// Provides: {"CheckLintNameResult"}
// Dependencies: {}
# [derive (Debug)] pub enum CheckLintNameResult < 'a > { Ok (& 'a [LintId]) , # [doc = " Lint doesn't exist. Potentially contains a suggestion for a correct lint name."] NoLint (Option < (Symbol , bool) >) , # [doc = " The lint refers to a tool that has not been registered."] NoTool , # [doc = " The lint has been renamed to a new name."] Renamed (String) , # [doc = " The lint has been removed due to the given reason."] Removed (String) , # [doc = " The lint is from a tool. The `LintId` will be returned as if it were a"] # [doc = " rustc lint. The `Option<String>` indicates if the lint has been"] # [doc = " renamed."] Tool (& 'a [LintId] , Option < String >) , # [doc = " The lint is from a tool. Either the lint does not exist in the tool or"] # [doc = " the code was not compiled with the tool and therefore the lint was"] # [doc = " never added to the `LintStore`."] MissingTool , }
};
}
