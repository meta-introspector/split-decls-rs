macro_rules! deps {
    () => {
        ForLoopsOverFalliblesLoopSub!();
        ForLoopsOverFalliblesQuestionMark!();
        ForLoopsOverFalliblesSuggestion!();
    };
}

macro_rules! ForLoopsOverFalliblesDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_for_loops_over_fallibles)] pub (crate) struct ForLoopsOverFalliblesDiag < 'a > { pub article : & 'static str , pub ref_prefix : & 'static str , pub ty : & 'static str , # [subdiagnostic] pub sub : ForLoopsOverFalliblesLoopSub < 'a > , # [subdiagnostic] pub question_mark : Option < ForLoopsOverFalliblesQuestionMark > , # [subdiagnostic] pub suggestion : ForLoopsOverFalliblesSuggestion < 'a > , }
    };
}

ForLoopsOverFalliblesDiag!()