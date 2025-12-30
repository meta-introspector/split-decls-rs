// Generated macro for add_lint (function)
macro_rules! Depcrate_new_lintadd_lint {
() => {
// Module: crate::new_lint
// Provides: {"add_lint"}
// Dependencies: {}
fn add_lint (lint : & LintData < '_ > , enable_msrv : bool) -> io :: Result < () > { let path = "clippy_lints/src/lib.rs" ; let mut lib_rs = fs :: read_to_string (path) . context ("reading") ? ; let (comment , ctor_arg) = if lint . pass == Pass :: Late { ("// add late passes here" , "_") } else { ("// add early passes here" , "") } ; let comment_start = lib_rs . find (comment) . expect ("Couldn't find comment") ; let module_name = lint . name ; let camel_name = to_camel_case (lint . name) ; let new_lint = if enable_msrv { format ! ("Box::new(move |{ctor_arg}| Box::new({module_name}::{camel_name}::new(conf))),\n        " ,) } else { format ! ("Box::new(|{ctor_arg}| Box::new({module_name}::{camel_name})),\n        " ,) } ; lib_rs . insert_str (comment_start , & new_lint) ; fs :: write (path , lib_rs) . context ("writing") }
};
}
