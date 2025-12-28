macro_rules! OverflowingIntHelp {
    () => {
        # [derive (Subdiagnostic)] # [help (lint_help)] pub (crate) struct OverflowingIntHelp < 'a > { pub suggestion_ty : & 'a str , }
    };
}

OverflowingIntHelp!();