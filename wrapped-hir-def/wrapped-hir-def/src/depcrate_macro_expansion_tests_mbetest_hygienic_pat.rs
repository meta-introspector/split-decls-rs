// Generated macro for test_hygienic_pat (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_hygienic_pat {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_hygienic_pat"}
// Dependencies: {}
# [test] fn test_hygienic_pat () { check (r#"
//- /new.rs crate:new deps:old edition:2015
old::make!();
fn main() {
    matches!(0, 0 | 1 if true);
}
//- /old.rs crate:old edition:2021
#[macro_export]
macro_rules! make {
    () => {
        macro_rules! matches {
            ($expression:expr, $pattern:pat if $guard:expr ) => {
                match $expression {
                    $pattern if $guard => true,
                    _ => false
                }
            };
        }
    }
}
 "# , expect ! [[r#"
macro_rules !matches {
    ($expression: expr, $pattern: pat if $guard: expr) = > {
        match $expression {
            $pattern if $guard = > true , _ = > false
        }
    }
    ;
}
fn main() {
    match 0 {
        0|1 if true =>true , _=>false
    };
}
"#]] ,) ; check (r#"
//- /new.rs crate:new deps:old edition:2021
old::make!();
fn main() {
    matches/*+errors*/!(0, 0 | 1 if true);
}
//- /old.rs crate:old edition:2015
#[macro_export]
macro_rules! make {
    () => {
        macro_rules! matches {
            ($expression:expr, $pattern:pat if $guard:expr ) => {
                match $expression {
                    $pattern if $guard => true,
                    _ => false
                }
            };
        }
    }
}
 "# , expect ! [[r#"
macro_rules !matches {
    ($expression: expr, $pattern: pat if $guard: expr) = > {
        match $expression {
            $pattern if $guard = > true , _ = > false
        }
    }
    ;
}
fn main() {
    /* error: unexpected token in input *//* parse error: expected expression */
/* parse error: expected FAT_ARROW */
/* parse error: expected `,` */
/* parse error: expected pattern */
match 0 {
        0 if $guard=>true , _=>false
    };
}
"#]] ,) ; }
};
}
