// Generated macro for regression_10042 (function)
macro_rules! Depcrate_context_testsregression_10042 {
() => {
// Module: crate::context::tests
// Provides: {"regression_10042"}
// Dependencies: {}
# [test] fn regression_10042 () { completion_list (r#"
macro_rules! preset {
    ($($x:ident)&&*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x.into());
            )*
            v
        }
    };
}

fn foo() {
    preset!(foo$0);
}
"# ,) ; }
};
}
