macro_rules! test_assert_expand_2015 {
    () => {
        # [test] fn test_assert_expand_2015 () { check (r#"
//- minicore: assert
//- /main.rs edition:2015
fn main() {
    assert!(true, "{} {:?}", arg1(a, b, c), arg2);
}
"# , expect ! [[r#"
fn main() {
     {
        if !(true ) {
            $crate::panic::panic_2021!("{} {:?}", arg1(a, b, c), arg2);
        }
    };
}
"#]] ,) ; }
    };
}

test_assert_expand_2015!();