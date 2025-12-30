// Generated macro for CheckNameUnknownTool (struct)
macro_rules! Depcrate_errorsCheckNameUnknownTool {
() => {
// Module: crate::errors
// Provides: {"CheckNameUnknownTool"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (lint_check_name_unknown_tool , code = E0602)] pub (crate) struct CheckNameUnknownTool < 'a > { pub tool_name : Symbol , # [subdiagnostic] pub sub : RequestedLevel < 'a > , }
};
}
