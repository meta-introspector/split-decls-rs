// Generated macro for find_lint_decls (function)
macro_rules! Depcrate_update_lintsfind_lint_decls {
() => {
// Module: crate::update_lints
// Provides: {"find_lint_decls"}
// Dependencies: {}
# [doc = " Finds all lint declarations (`declare_clippy_lint!`)"] # [must_use] pub fn find_lint_decls () -> Vec < Lint > { let mut lints = Vec :: with_capacity (1000) ; let mut contents = String :: new () ; for e in expect_action (fs :: read_dir (".") , ErrAction :: Read , ".") { let e = expect_action (e , ErrAction :: Read , ".") ; if ! expect_action (e . file_type () , ErrAction :: Read , ".") . is_dir () { continue ; } let Ok (mut name) = e . file_name () . into_string () else { continue ; } ; if name . starts_with ("clippy_lints") && name != "clippy_lints_internal" { name . push_str ("/src") ; for (file , module) in read_src_with_module (name . as_ref ()) { parse_clippy_lint_decls (file . path () , File :: open_read_to_cleared_string (file . path () , & mut contents) , & module , & mut lints ,) ; } } } lints . sort_by (| lhs , rhs | lhs . name . cmp (& rhs . name)) ; lints }
};
}
