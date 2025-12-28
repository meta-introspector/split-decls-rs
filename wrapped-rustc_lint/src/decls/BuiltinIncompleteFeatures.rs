macro_rules! deps {
    () => {
        BuiltinIncompleteFeaturesHelp!();
        BuiltinFeatureIssueNote!();
    };
}

macro_rules! BuiltinIncompleteFeatures {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_builtin_incomplete_features)] pub (crate) struct BuiltinIncompleteFeatures { pub name : Symbol , # [subdiagnostic] pub note : Option < BuiltinFeatureIssueNote > , # [subdiagnostic] pub help : Option < BuiltinIncompleteFeaturesHelp > , }
    };
}

BuiltinIncompleteFeatures!()