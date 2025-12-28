macro_rules! test_vertical_bar_with_pat_param {
    () => {
        # [test] fn test_vertical_bar_with_pat_param () { check (r#"
macro_rules! m { (|$pat:pat_param| ) => { ok!(); } }
m! { |x| }
 "# , expect ! [[r#"
macro_rules! m { (|$pat:pat_param| ) => { ok!(); } }
ok!();
 "#]] ,) ; }
    };
}

test_vertical_bar_with_pat_param!()