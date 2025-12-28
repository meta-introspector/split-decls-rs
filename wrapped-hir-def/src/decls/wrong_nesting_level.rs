macro_rules! wrong_nesting_level {
    () => {
        # [test] fn wrong_nesting_level () { check (r#"
macro_rules! m {
    ($($i:ident);*) => ($i)
}
m!{a}
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident);*) => ($i)
}
/* error: expected simple binding, found nested binding `i` */
"#]] ,) ; }
    };
}

wrong_nesting_level!();