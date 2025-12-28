macro_rules! deps {
    () => {
        NonUpperCaseGlobalSub!();
        NonUpperCaseGlobalSubTool!();
    };
}

macro_rules! NonUpperCaseGlobal {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_non_upper_case_global)] pub (crate) struct NonUpperCaseGlobal < 'a > { pub sort : & 'a str , pub name : & 'a str , # [subdiagnostic] pub sub : NonUpperCaseGlobalSub , # [subdiagnostic] pub usages : Vec < NonUpperCaseGlobalSubTool > , }
    };
}

NonUpperCaseGlobal!()