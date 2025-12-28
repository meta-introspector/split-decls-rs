macro_rules! no_completions_in_comments {
    () => {
        # [test] fn no_completions_in_comments () { assert_eq ! (completion_list (r#"
fn test() {
let x = 2; // A comment$0
}
"# ,) , String :: new () ,) ; assert_eq ! (completion_list (r#"
/*
Some multi-line comment$0
*/
"# ,) , String :: new () ,) ; assert_eq ! (completion_list (r#"
/// Some doc comment
/// let test$0 = 1
"# ,) , String :: new () ,) ; }
    };
}

no_completions_in_comments!()