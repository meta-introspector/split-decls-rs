// Generated macro for apply_replace_expression (function)
macro_rules! Depcrateapply_replace_expression {
() => {
// Module: crate
// Provides: {"apply_replace_expression"}
// Dependencies: {}
fn apply_replace_expression (ast : & mut File , details : & ReplaceExpressionDetails) -> Result < () > { let mut visitor = ExpressionReplacer { function_name : & details . function_name , old_snippet : syn :: parse_str (& details . old_code_snippet) . context ("Failed to parse old code snippet") ? , new_snippet : syn :: parse_str (& details . new_code_snippet) . context ("Failed to parse new code snippet") ? , replaced_count : 0 , } ; visitor . visit_file_mut (ast) ; if visitor . replaced_count > 0 { println ! ("  Replaced {} occurrences of '{}' with '{}' in function '{}'." , visitor . replaced_count , details . old_code_snippet , details . new_code_snippet , details . function_name) ; } else { println ! ("  Warning: No occurrences of '{}' found in function '{}' for replacement." , details . old_code_snippet , details . function_name) ; } Ok (()) }
};
}
