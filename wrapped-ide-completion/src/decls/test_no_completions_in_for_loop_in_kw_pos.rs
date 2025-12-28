macro_rules! test_no_completions_in_for_loop_in_kw_pos {
    () => {
        # [test] fn test_no_completions_in_for_loop_in_kw_pos () { assert_eq ! (completion_list (r#"fn foo() { for i i$0 }"#) , String :: new ()) ; assert_eq ! (completion_list (r#"fn foo() { for i in$0 }"#) , String :: new ()) ; }
    };
}

test_no_completions_in_for_loop_in_kw_pos!()