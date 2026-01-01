/* FP:print.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_print_USE_0001
/* FP:print.rs-0002 */ use std :: fmt ;
/* FP:print.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_print_USE_0002
/* FP:print.rs-0004 */ use std :: io :: { self , Write as _ } ;
/* FP:print.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_print_MACRO_0003
/* FP:print.rs-0006 */ macro_rules ! safe_print { ($ ($ arg : tt) *) => { { $ crate :: print :: print (std :: format_args ! ($ ($ arg) *)) ; } } ; }
/* FP:print.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_print_MACRO_0004
/* FP:print.rs-0008 */ macro_rules ! safe_println { ($ ($ arg : tt) *) => { safe_print ! ("{}\n" , std :: format_args ! ($ ($ arg) *)) } ; }
/* FP:print.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_driver_impl_src_print_FN_0005
/* FP:print.rs-0010 */ pub (crate) fn print (args : fmt :: Arguments < '_ >) { if let Err (_) = io :: stdout () . write_fmt (args) { crate :: rustc_errors :: FatalError . raise () ; } }