macro_rules! deps {
    () => {
        AmbiguousNegativeLiteralsCurrentBehaviorSuggestion!();
        AmbiguousNegativeLiteralsNegativeLiteralSuggestion!();
    };
}

macro_rules! AmbiguousNegativeLiteralsDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_ambiguous_negative_literals)] # [note (lint_example)] pub (crate) struct AmbiguousNegativeLiteralsDiag { # [subdiagnostic] pub negative_literal : AmbiguousNegativeLiteralsNegativeLiteralSuggestion , # [subdiagnostic] pub current_behavior : AmbiguousNegativeLiteralsCurrentBehaviorSuggestion , }
    };
}

AmbiguousNegativeLiteralsDiag!()