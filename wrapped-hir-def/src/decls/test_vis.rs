macro_rules! test_vis {
    () => {
        # [test] fn test_vis () { check (r#"
macro_rules! m {
    ($vis:vis $name:ident) => { $vis fn $name() {} }
}
m!(pub foo);
m!(foo);
"# , expect ! [[r#"
macro_rules! m {
    ($vis:vis $name:ident) => { $vis fn $name() {} }
}
pub fn foo() {}
fn foo() {}
"#]] ,) ; }
    };
}

test_vis!()