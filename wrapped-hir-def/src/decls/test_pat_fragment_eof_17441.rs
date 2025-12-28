macro_rules! test_pat_fragment_eof_17441 {
    () => {
        # [test] fn test_pat_fragment_eof_17441 () { check (r#"
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? ) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
fn f() {
    matches!(0, 10..);
    matches!(0, 10.. if true);
}
 "# , expect ! [[r#"
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? ) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
fn f() {
    match 0 {
        10.. =>true , _=>false
    };
    match 0 {
        10..if true =>true , _=>false
    };
}
 "#]] ,) ; }
    };
}

test_pat_fragment_eof_17441!();