// Generated macro for _assert_snapshot_base (macro)
macro_rules! Depcrate_macros_assert_snapshot_base {
() => {
// Module: crate::macros
// Provides: {"_assert_snapshot_base"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! _assert_snapshot_base { (transform =$ transform : expr , $ ($ arg : expr) ,*, @$ snapshot : literal $ (,) ?) => { $ crate :: _assert_snapshot_base ! (transform = $ transform , # [allow (clippy :: needless_raw_string_hashes)] $ crate :: _macro_support :: InlineValue ($ snapshot) , $ ($ arg) ,*) } ; (transform =$ transform : expr , $ name : expr , $ value : expr $ (,) ?) => { $ crate :: _assert_snapshot_base ! (transform = $ transform , $ name , $ value , stringify ! ($ value)) } ; (transform =$ transform : expr , $ value : expr $ (,) ?) => { $ crate :: _assert_snapshot_base ! (transform = $ transform , $ crate :: _macro_support :: AutoName , $ value) } ; (transform =$ transform : expr , $ name : expr , $ value : expr , $ debug_expr : expr $ (,) ?) => { $ crate :: _macro_support :: assert_snapshot (($ name , # [allow (clippy :: redundant_closure_call)] $ transform (&$ value) . as_str () ,) . into () , $ crate :: _get_workspace_root ! () . as_path () , $ crate :: _function_name ! () , $ crate :: _macro_support :: module_path ! () , $ crate :: _macro_support :: file ! () , $ crate :: _macro_support :: line ! () , $ debug_expr ,) . unwrap () } ; }
};
}
