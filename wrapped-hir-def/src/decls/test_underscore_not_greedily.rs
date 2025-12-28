macro_rules! test_underscore_not_greedily {
    () => {
        # [test] fn test_underscore_not_greedily () { check (r#"
// `_` overlaps with `$a:ident` but rustc matches it under the `_` token.
macro_rules! m1 {
    ($($a:ident)* _) => { ok!(); }
}
m1![a b c d _];

// `_ => ou` overlaps with `$a:expr => $b:ident` but rustc matches it under `_ => $c:expr`.
macro_rules! m2 {
    ($($a:expr => $b:ident)* _ => $c:expr) => { ok!(); }
}
m2![a => b c => d _ => ou]
"# , expect ! [[r#"
// `_` overlaps with `$a:ident` but rustc matches it under the `_` token.
macro_rules! m1 {
    ($($a:ident)* _) => { ok!(); }
}
ok!();

// `_ => ou` overlaps with `$a:expr => $b:ident` but rustc matches it under `_ => $c:expr`.
macro_rules! m2 {
    ($($a:expr => $b:ident)* _ => $c:expr) => { ok!(); }
}
/* error: unexpected token in input */ok!();
"#]] ,) ; }
    };
}

test_underscore_not_greedily!();