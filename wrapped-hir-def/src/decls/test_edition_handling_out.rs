macro_rules! test_edition_handling_out {
    () => {
        # [test] fn test_edition_handling_out () { check (r#"
//- /main.rs crate:main deps:old edition:2021
macro_rules! r#try {
    ($it:expr) => {
        $it?
    };
}
fn f() {
    old::invoke_bare_try!(0);
}
//- /old.rs crate:old edition:2015
#[macro_export]
macro_rules! invoke_bare_try {
    ($it:expr) => {
        try!($it)
    };
}
 "# , expect ! [[r#"
macro_rules! r#try {
    ($it:expr) => {
        $it?
    };
}
fn f() {
    try!(0);
}
"#]] ,) ; }
    };
}

test_edition_handling_out!();