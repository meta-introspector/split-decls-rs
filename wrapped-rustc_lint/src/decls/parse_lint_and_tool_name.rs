macro_rules! parse_lint_and_tool_name {
    () => {
        pub (crate) fn parse_lint_and_tool_name (lint_name : & str) -> (Option < Symbol > , & str) { match lint_name . split_once ("::") { Some ((tool_name , lint_name)) => { let tool_name = Symbol :: intern (tool_name) ; (Some (tool_name) , lint_name) } None => (None , lint_name) , } }
    };
}

parse_lint_and_tool_name!()