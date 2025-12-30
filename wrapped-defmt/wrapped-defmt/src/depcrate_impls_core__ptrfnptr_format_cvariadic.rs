// Generated macro for fnptr_format_cvariadic (macro)
macro_rules! Depcrate_impls_core__ptrfnptr_format_cvariadic {
() => {
// Module: crate::impls::core_::ptr
// Provides: {"fnptr_format_cvariadic"}
// Dependencies: {}
# [cfg (c_variadic)] macro_rules ! fnptr_format_cvariadic { ($ ($ Arg : ident) ,+) => { impl < Ret , $ ($ Arg) ,*> Format for extern "C" fn ($ ($ Arg) ,* , ...) -> Ret { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}" , (* self as usize) as * const ()) } } impl < Ret , $ ($ Arg) ,*> Format for unsafe extern "C" fn ($ ($ Arg) ,* , ...) -> Ret { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}" , (* self as usize) as * const ()) } } } ; () => { } ; }
};
}
