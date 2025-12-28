macro_rules! eager_expands_with_unresolved_within {
    () => {
        # [test] fn eager_expands_with_unresolved_within () { check (r#"
#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {}
macro_rules! identity {
    ($tt:tt) => {
        $tt
    }
}

fn main(foo: ()) {
    concat!("hello", identity!("world"), unresolved!(), identity!("!"));
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
#[macro_export]
macro_rules! concat {}
macro_rules! identity {
    ($tt:tt) => {
        $tt
    }
}

fn main(foo: ()) {
    /* error: unresolved macro unresolved */"helloworld!";
}
"##]] ,) ; }
    };
}

eager_expands_with_unresolved_within!();