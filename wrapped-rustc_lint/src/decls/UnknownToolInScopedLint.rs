macro_rules! UnknownToolInScopedLint {
    () => {
        # [derive (Diagnostic)] # [diag (lint_unknown_tool_in_scoped_lint , code = E0710)] pub (crate) struct UnknownToolInScopedLint { # [primary_span] pub span : Option < Span > , pub tool_name : Symbol , pub lint_name : String , # [help] pub is_nightly_build : bool , }
    };
}

UnknownToolInScopedLint!();