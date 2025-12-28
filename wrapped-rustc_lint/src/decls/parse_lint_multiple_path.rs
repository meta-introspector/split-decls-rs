macro_rules! parse_lint_multiple_path {
    () => {
        # [test] fn parse_lint_multiple_path () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("clippy::foo::bar") , (Some (Symbol :: intern ("clippy")) , "foo::bar")) }) ; }
    };
}

parse_lint_multiple_path!()