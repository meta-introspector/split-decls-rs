macro_rules! test_edition_handling_in {
    () => {
        # [test] fn test_edition_handling_in () { check (r#"
//- /main.rs crate:main deps:old edition:2021
fn f() {
    old::parse_try_old!(try!{});
}
//- /old.rs crate:old edition:2015
#[macro_export]
macro_rules! parse_try_old {
    ($it:expr) => {};
}
 "# , expect ! [[r#"
fn f() {
    ;
}
"#]] ,) ; }
    };
}

test_edition_handling_in!()