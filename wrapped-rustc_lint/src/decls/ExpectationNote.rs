macro_rules! ExpectationNote {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_rationale)] pub (crate) struct ExpectationNote { pub rationale : Symbol , }
    };
}

ExpectationNote!()