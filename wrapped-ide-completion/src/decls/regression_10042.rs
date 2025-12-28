macro_rules! regression_10042 {
    () => {
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

regression_10042!()