// Generated macro for create_new_without_default_suggest_msg (function)
macro_rules! Depcrate_new_without_defaultcreate_new_without_default_suggest_msg {
() => {
// Module: crate::new_without_default
// Provides: {"create_new_without_default_suggest_msg"}
// Dependencies: {}
fn create_new_without_default_suggest_msg (attrs_sugg : & str , self_type_snip : & str , generics_sugg : & str , where_clause_sugg : & str ,) -> String { # [rustfmt :: skip] format ! ("{attrs_sugg}impl{generics_sugg} Default for {self_type_snip}{where_clause_sugg} {{
    fn default() -> Self {{
        Self::new()
    }}
}}") }
};
}
