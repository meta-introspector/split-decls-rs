macro_rules! deps {
    () => {
        OverflowingIntHelp!();
    };
}

macro_rules! OverflowingInt {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_overflowing_int)] # [note] pub (crate) struct OverflowingInt < 'a > { pub ty : & 'a str , pub lit : String , pub min : i128 , pub max : u128 , # [subdiagnostic] pub help : Option < OverflowingIntHelp < 'a > > , }
    };
}

OverflowingInt!();