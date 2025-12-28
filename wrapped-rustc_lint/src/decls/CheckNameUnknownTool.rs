macro_rules! deps {
    () => {
        RequestedLevel!();
    };
}

macro_rules! CheckNameUnknownTool {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (lint_check_name_unknown_tool , code = E0602)] pub (crate) struct CheckNameUnknownTool < 'a > { pub tool_name : Symbol , # [subdiagnostic] pub sub : RequestedLevel < 'a > , }
    };
}

CheckNameUnknownTool!();