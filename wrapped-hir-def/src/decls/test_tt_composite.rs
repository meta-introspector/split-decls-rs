macro_rules! test_tt_composite {
    () => {
        # [test] fn test_tt_composite () { check (r#"
macro_rules! m { ($tt:tt) => { ok!(); } }
m! { => }
m! { = > }
"# , expect ! [[r#"
macro_rules! m { ($tt:tt) => { ok!(); } }
ok!();
/* error: leftover tokens */ok!();
"#]] ,) ; }
    };
}

test_tt_composite!()