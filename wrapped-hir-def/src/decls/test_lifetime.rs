macro_rules! test_lifetime {
    () => {
        # [test] fn test_lifetime () { check (r#"
macro_rules! m {
    ($lt:lifetime) => { struct Ref<$lt>{ s: &$ lt str } }
}
m! {'a}
"# , expect ! [[r#"
macro_rules! m {
    ($lt:lifetime) => { struct Ref<$lt>{ s: &$ lt str } }
}
struct Ref<'a> {
    s: &'a str
}
"#]] ,) ; }
    };
}

test_lifetime!();