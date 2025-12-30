// Generated macro for get_lint_declaration (function)
macro_rules! Depcrate_new_lintget_lint_declaration {
() => {
// Module: crate::new_lint
// Provides: {"get_lint_declaration"}
// Dependencies: {}
fn get_lint_declaration (version : Version , name_upper : & str , category : & str) -> String { let justification_heading = if category == "restriction" { "Why restrict this?" } else { "Why is this bad?" } ; formatdoc ! (r#"
            declare_clippy_lint! {{
                /// ### What it does
                ///
                /// ### {justification_heading}
                ///
                /// ### Example
                /// ```no_run
                /// // example code where clippy issues a warning
                /// ```
                /// Use instead:
                /// ```no_run
                /// // example code which does not raise clippy warning
                /// ```
                #[clippy::version = "{}"]
                pub {name_upper},
                {category},
                "default lint description"
            }}"# , version . rust_display () ,) }
};
}
