// Generated macro for create_lint (function)
macro_rules! Depcrate_new_lintcreate_lint {
() => {
// Module: crate::new_lint
// Provides: {"create_lint"}
// Dependencies: {}
fn create_lint (lint : & LintData < '_ > , enable_msrv : bool) -> io :: Result < () > { if let Some (ty) = lint . ty { create_lint_for_ty (lint , enable_msrv , ty) } else { let lint_contents = get_lint_file_contents (lint , enable_msrv) ; let lint_path = format ! ("clippy_lints/src/{}.rs" , lint . name) ; write_file (& lint_path , lint_contents . as_bytes ()) ? ; println ! ("Generated lint file: `{lint_path}`") ; Ok (()) } }
};
}
