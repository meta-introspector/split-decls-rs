macro_rules! deps {
    () => {
        AmbiguousNegativeLiteralsNegativeLiteralSuggestion!();
        AmbiguousNegativeLiteralsCurrentBehaviorSuggestion!();
    };
}

macro_rules! AmbiguousNegativeLiteralsDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_ambiguous_negative_literals)] # [note (lint_example)] pub (crate) struct AmbiguousNegativeLiteralsDiag { # [subdiagnostic] pub negative_literal : AmbiguousNegativeLiteralsNegativeLiteralSuggestion , # [subdiagnostic] pub current_behavior : AmbiguousNegativeLiteralsCurrentBehaviorSuggestion , }
    };
}

AmbiguousNegativeLiteralsDiag!();