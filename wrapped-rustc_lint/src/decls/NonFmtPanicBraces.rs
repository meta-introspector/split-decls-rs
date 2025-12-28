macro_rules! NonFmtPanicBraces {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_non_fmt_panic_braces)] # [note] pub (crate) struct NonFmtPanicBraces { pub count : usize , # [suggestion (code = "\"{{}}\", " , applicability = "machine-applicable")] pub suggestion : Option < Span > , }
    };
}

NonFmtPanicBraces!()