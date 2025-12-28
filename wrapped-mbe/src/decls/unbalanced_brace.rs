macro_rules! unbalanced_brace {
    () => {
        # [test] fn unbalanced_brace () { check (Edition :: CURRENT , Edition :: CURRENT , r#"
() => { { }
"# , r#""# , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..0#ROOT2024 1:Root[0000, 0]@0..0#ROOT2024
              SUBTREE {} 0:Root[0000, 0]@9..10#ROOT2024 0:Root[0000, 0]@11..12#ROOT2024

            {}"#]] ,) ; }
    };
}

unbalanced_brace!();