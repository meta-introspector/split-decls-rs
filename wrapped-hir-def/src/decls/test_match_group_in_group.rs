macro_rules! test_match_group_in_group {
    () => {
        # [test] fn test_match_group_in_group () { check (r#"
macro_rules! m {
    [ $( ( $($i:ident)* ) )* ] => [ ok![$( ( $($i)* ) )*]; ]
}
m! ( (a b) );
"# , expect ! [[r#"
macro_rules! m {
    [ $( ( $($i:ident)* ) )* ] => [ ok![$( ( $($i)* ) )*]; ]
}
ok![(a b)];
"#]] ,) }
    };
}

test_match_group_in_group!()