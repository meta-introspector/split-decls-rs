macro_rules! deps {
    () => {
        UnusedDelimSuggestion!();
    };
}

macro_rules! UnusedDelim {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unused_delim)] pub (crate) struct UnusedDelim < 'a > { pub delim : & 'static str , pub item : & 'a str , # [subdiagnostic] pub suggestion : Option < UnusedDelimSuggestion > , }
    };
}

UnusedDelim!()