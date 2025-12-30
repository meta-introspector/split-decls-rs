// Generated macro for tests (module)
macro_rules! Depcrate_update_lintstests {
() => {
// Module: crate::update_lints
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_parse_clippy_lint_decls () { static CONTENTS : & str = r#"
            declare_clippy_lint! {
                #[clippy::version = "Hello Clippy!"]
                pub PTR_ARG,
                style,
                "really long \
                text"
            }

            declare_clippy_lint!{
                #[clippy::version = "Test version"]
                pub DOC_MARKDOWN,
                pedantic,
                "single line"
            }
        "# ; let mut result = Vec :: new () ; parse_clippy_lint_decls ("" . as_ref () , CONTENTS , "module_name" , & mut result) ; for r in & mut result { r . declaration_range = Range :: default () ; } let expected = vec ! [Lint { name : "ptr_arg" . into () , group : "style" . into () , module : "module_name" . into () , path : PathBuf :: new () , declaration_range : Range :: default () , } , Lint { name : "doc_markdown" . into () , group : "pedantic" . into () , module : "module_name" . into () , path : PathBuf :: new () , declaration_range : Range :: default () , } ,] ; assert_eq ! (expected , result) ; } }
};
}
