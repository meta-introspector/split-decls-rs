macro_rules! deps {
    () => {
        DeprecatedWhereClauseLocationSugg!();
    };
}

macro_rules! DeprecatedWhereClauseLocation {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_deprecated_where_clause_location)] # [note] pub (crate) struct DeprecatedWhereClauseLocation { # [subdiagnostic] pub suggestion : DeprecatedWhereClauseLocationSugg , }
    };
}

DeprecatedWhereClauseLocation!()