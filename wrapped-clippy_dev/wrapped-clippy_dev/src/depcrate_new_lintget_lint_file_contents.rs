// Generated macro for get_lint_file_contents (function)
macro_rules! Depcrate_new_lintget_lint_file_contents {
() => {
// Module: crate::new_lint
// Provides: {"get_lint_file_contents"}
// Dependencies: {}
fn get_lint_file_contents (lint : & LintData < '_ > , enable_msrv : bool) -> String { let mut result = String :: new () ; let (pass_type , pass_lifetimes , pass_import , context_import) = match lint . pass { Pass :: Early => ("EarlyLintPass" , "" , "use rustc_ast::ast::*;" , "EarlyContext") , Pass :: Late => ("LateLintPass" , "<'_>" , "use rustc_hir::*;" , "LateContext") , } ; let (msrv_ty , msrv_ctor , extract_msrv) = match lint . pass { Pass :: Early => ("MsrvStack" , "MsrvStack::new(conf.msrv)" , "\n    extract_msrv_attr!();\n" ,) , Pass :: Late => ("Msrv" , "conf.msrv" , "") , } ; let lint_name = lint . name ; let category = lint . category ; let name_camel = to_camel_case (lint . name) ; let name_upper = lint_name . to_uppercase () ; if enable_msrv { let _ : fmt :: Result = writedoc ! (result , r"
            use clippy_utils::msrvs::{{self, {msrv_ty}}};
            use clippy_config::Conf;
            {pass_import}
            use rustc_lint::{{{context_import}, {pass_type}}};
            use rustc_session::impl_lint_pass;

        ") ; } else { let _ : fmt :: Result = writedoc ! (result , r"
            {pass_import}
            use rustc_lint::{{{context_import}, {pass_type}}};
            use rustc_session::declare_lint_pass;

        ") ; } let _ : fmt :: Result = writeln ! (result , "{}" , get_lint_declaration (lint . clippy_version , & name_upper , category)) ; if enable_msrv { let _ : fmt :: Result = writedoc ! (result , r"
            pub struct {name_camel} {{
                msrv: {msrv_ty},
            }}

            impl {name_camel} {{
                pub fn new(conf: &'static Conf) -> Self {{
                    Self {{ msrv: {msrv_ctor} }}
                }}
            }}

            impl_lint_pass!({name_camel} => [{name_upper}]);

            impl {pass_type}{pass_lifetimes} for {name_camel} {{{extract_msrv}}}

            // TODO: Add MSRV level to `clippy_config/src/msrvs.rs` if needed.
            // TODO: Update msrv config comment in `clippy_config/src/conf.rs`
        ") ; } else { let _ : fmt :: Result = writedoc ! (result , r"
            declare_lint_pass!({name_camel} => [{name_upper}]);

            impl {pass_type}{pass_lifetimes} for {name_camel} {{}}
        ") ; } result }
};
}
