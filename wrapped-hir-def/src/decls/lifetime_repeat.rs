macro_rules! lifetime_repeat {
    () => {
        # [test] fn lifetime_repeat () { check (r#"
macro_rules! m {
    ($($x:expr)'a*) => (stringify!($($x)'b*));
}
fn f() {
    let _ = m!(0 'a 1 'a 2);
}
    "# , expect ! [[r#"
macro_rules! m {
    ($($x:expr)'a*) => (stringify!($($x)'b*));
}
fn f() {
    let _ = stringify!(0 'b 1 'b 2);
}
    "#]] ,) ; }
    };
}

lifetime_repeat!()