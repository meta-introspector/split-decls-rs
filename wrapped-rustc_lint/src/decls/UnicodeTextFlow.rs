macro_rules! deps {
    () => {
        UnicodeCharNoteSub!();
        UnicodeTextFlowSuggestion!();
    };
}

macro_rules! UnicodeTextFlow {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unicode_text_flow)] # [note] pub (crate) struct UnicodeTextFlow { # [label] pub comment_span : Span , # [subdiagnostic] pub characters : Vec < UnicodeCharNoteSub > , # [subdiagnostic] pub suggestions : Option < UnicodeTextFlowSuggestion > , pub num_codepoints : usize , }
    };
}

UnicodeTextFlow!();