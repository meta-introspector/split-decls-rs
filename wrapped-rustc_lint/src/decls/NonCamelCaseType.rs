macro_rules! deps {
    () => {
        NonCamelCaseTypeSub!();
    };
}

macro_rules! NonCamelCaseType {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_non_camel_case_type)] pub (crate) struct NonCamelCaseType < 'a > { pub sort : & 'a str , pub name : & 'a str , # [subdiagnostic] pub sub : NonCamelCaseTypeSub , }
    };
}

NonCamelCaseType!();