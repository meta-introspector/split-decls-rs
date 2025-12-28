macro_rules! test_tt_composite2 {
    () => {
        # [test] fn test_tt_composite2 () { check (r#"
macro_rules! m { ($($tt:tt)*) => { abs!(=> $($tt)*); } }
m! {#}
"# , expect ! [[r#"
macro_rules! m { ($($tt:tt)*) => { abs!(=> $($tt)*); } }
abs!( = > #);
"#]] ,) ; }
    };
}

test_tt_composite2!()