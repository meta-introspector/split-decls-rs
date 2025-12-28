macro_rules! test_literal {
    () => {
        # [test] fn test_literal () { check (r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $type = $ lit; };
}
m!(u8, 0);
"# , expect ! [[r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $type = $ lit; };
}
const VALUE: u8 = 0;
"#]] ,) ; check (r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $ type = $ lit; };
}
m!(i32, -1);
"# , expect ! [[r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $ type = $ lit; };
}
const VALUE: i32 = -1;
"#]] ,) ; }
    };
}

test_literal!();