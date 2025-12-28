macro_rules! BadDeriveLitHelp {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum BadDeriveLitHelp { # [help (builtin_macros_str_lit)] StrLit { sym : Symbol } , # [help (builtin_macros_other)] Other , }
    };
}

BadDeriveLitHelp!()