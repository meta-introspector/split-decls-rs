macro_rules! UnsupportedGroup {
    () => {
        # [derive (Diagnostic)] # [diag (lint_unsupported_group , code = E0602)] pub (crate) struct UnsupportedGroup { pub lint_group : String , }
    };
}

UnsupportedGroup!()