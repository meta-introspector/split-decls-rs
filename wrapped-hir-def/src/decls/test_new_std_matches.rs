macro_rules! test_new_std_matches {
    () => {
        # [test] fn test_new_std_matches () { check (r#"
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
fn main() {
    matches!(0, 0 | 1 if true);
}
 "# , expect ! [[r#"
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
fn main() {
    match 0 {
        0|1 if true =>true , _=>false
    };
}
 "#]] ,) ; }
    };
}

test_new_std_matches!()