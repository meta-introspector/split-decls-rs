// Generated macro for fnptr_format_args (macro)
macro_rules! Depcrate_impls_core__ptrfnptr_format_args {
() => {
// Module: crate::impls::core_::ptr
// Provides: {"fnptr_format_args"}
// Dependencies: {}
macro_rules ! fnptr_format_args { ($ ($ Arg : ident) ,*) => { impl < Ret , $ ($ Arg) ,*> Format for extern "Rust" fn ($ ($ Arg) ,*) -> Ret { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}" , (* self as usize) as * const ()) } } impl < Ret , $ ($ Arg) ,*> Format for extern "C" fn ($ ($ Arg) ,*) -> Ret { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}" , (* self as usize) as * const ()) } } impl < Ret , $ ($ Arg) ,*> Format for unsafe extern "Rust" fn ($ ($ Arg) ,*) -> Ret { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}" , (* self as usize) as * const ()) } } impl < Ret , $ ($ Arg) ,*> Format for unsafe extern "C" fn ($ ($ Arg) ,*) -> Ret { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}" , (* self as usize) as * const ()) } } # [cfg (c_variadic)] fnptr_format_cvariadic ! { $ ($ Arg) ,* } } ; }
};
}
