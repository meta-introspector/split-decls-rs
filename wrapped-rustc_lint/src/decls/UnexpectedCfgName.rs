macro_rules! UnexpectedCfgName {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unexpected_cfg_name)] pub (crate) struct UnexpectedCfgName { # [subdiagnostic] pub code_sugg : unexpected_cfg_name :: CodeSuggestion , # [subdiagnostic] pub invocation_help : unexpected_cfg_name :: InvocationHelp , pub name : Symbol , }
    };
}

UnexpectedCfgName!();