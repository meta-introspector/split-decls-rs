// Generated macro for create_lint_for_ty (function)
macro_rules! Depcrate_new_lintcreate_lint_for_ty {
() => {
// Module: crate::new_lint
// Provides: {"create_lint_for_ty"}
// Dependencies: {}
fn create_lint_for_ty (lint : & LintData < '_ > , enable_msrv : bool , ty : & str) -> io :: Result < () > { match ty { "cargo" => assert_eq ! (lint . category , "cargo" , "Lints of type `cargo` must have the `cargo` category") , _ if lint . category == "cargo" => panic ! ("Lints of category `cargo` must have the `cargo` type") , _ => { } , } let ty_dir = PathBuf :: from (format ! ("clippy_lints/src/{ty}")) ; assert ! (ty_dir . exists () && ty_dir . is_dir () , "Directory `{}` does not exist!" , ty_dir . display ()) ; let lint_file_path = ty_dir . join (format ! ("{}.rs" , lint . name)) ; assert ! (! lint_file_path . exists () , "File `{}` already exists" , lint_file_path . display ()) ; let mod_file_path = ty_dir . join ("mod.rs") ; let context_import = setup_mod_file (& mod_file_path , lint) ? ; let (pass_lifetimes , msrv_ty , msrv_ref , msrv_cx) = match context_import { "LateContext" => ("<'_>" , "Msrv" , "" , "cx, ") , _ => ("" , "MsrvStack" , "&" , "") , } ; let name_upper = lint . name . to_uppercase () ; let mut lint_file_contents = String :: new () ; if enable_msrv { let _ : fmt :: Result = writedoc ! (lint_file_contents , r#"
                use clippy_utils::msrvs::{{self, {msrv_ty}}};
                use rustc_lint::{{{context_import}, LintContext}};

                use super::{name_upper};

                // TODO: Adjust the parameters as necessary
                pub(super) fn check(cx: &{context_import}{pass_lifetimes}, msrv: {msrv_ref}{msrv_ty}) {{
                    if !msrv.meets({msrv_cx}todo!("Add a new entry in `clippy_utils/src/msrvs`")) {{
                        return;
                    }}
                    todo!();
                }}
           "#) ; } else { let _ : fmt :: Result = writedoc ! (lint_file_contents , r"
                use rustc_lint::{{{context_import}, LintContext}};

                use super::{name_upper};

                // TODO: Adjust the parameters as necessary
                pub(super) fn check(cx: &{context_import}{pass_lifetimes}) {{
                    todo!();
                }}
           ") ; } write_file (lint_file_path . as_path () , lint_file_contents) ? ; println ! ("Generated lint file: `clippy_lints/src/{ty}/{}.rs`" , lint . name) ; println ! ("Be sure to add a call to `{}::check` in `clippy_lints/src/{ty}/mod.rs`!" , lint . name) ; Ok (()) }
};
}
