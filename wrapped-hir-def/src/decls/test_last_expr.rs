macro_rules! test_last_expr {
    () => {
        # [test] fn test_last_expr () { check (r#"
macro_rules! vec {
    ($($item:expr),*) => {{
            let mut v = Vec::new();
            $( v.push($item); )*
            v
    }};
}

fn f() {
    vec![1,2,3];
}
"# , expect ! [[r#"
macro_rules! vec {
    ($($item:expr),*) => {{
            let mut v = Vec::new();
            $( v.push($item); )*
            v
    }};
}

fn f() {
     {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v
    };
}
"#]] ,) ; }
    };
}

test_last_expr!()