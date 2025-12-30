// Generated macro for prefixed_export (macro)
macro_rules! Depcrate_prefixedprefixed_export {
() => {
// Module: crate::prefixed
// Provides: {"prefixed_export"}
// Dependencies: {}
# [deprecated = "`#[export_name]` creates problems and we will stop doing it."] # [cfg (not (any (all (target_arch = "aarch64" , target_endian = "little") , all (target_arch = "arm" , target_endian = "little") , target_arch = "x86" , target_arch = "x86_64")))] macro_rules ! prefixed_export { { $ (# [$ meta : meta]) * $ vis : vis unsafe extern "C" fn $ name : ident ($ ($ arg_pat : ident : $ arg_ty : ty) ,* $ (,) ?) $ body : block } => { prefixed_item ! { export_name $ name { $ (# [$ meta]) * $ vis unsafe extern "C" fn $ name ($ ($ arg_pat : $ arg_ty) ,*) $ body } } } ; }
};
}
