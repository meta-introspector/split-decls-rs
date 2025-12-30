// Generated macro for ExportJs (enum)
macro_rules! Depcrate_jsExportJs {
() => {
// Module: crate::js
// Provides: {"ExportJs"}
// Dependencies: {}
# [doc = " Different JS constructs that can be exported."] enum ExportJs < 'a > { # [doc = " A class of the form `class Name {...}`."] Class (& 'a str) , # [doc = " An anonymous function expression of the form `function(...) {...}`."] # [doc = ""] # [doc = " Note that the function name is not included in the string."] Function (& 'a str) , # [doc = " An arbitrary JS expression."] Expression (& 'a str) , }
};
}
