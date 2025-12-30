// Generated macro for impl_82 (impl)
macro_rules! Depcrate_codegenimpl_82 {
() => {
// Module: crate::codegen
// Provides: {"impl_82"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for Descriptor < '_ , T > { fn to_tokens (& self , tokens : & mut TokenStream) { thread_local ! { static DESCRIPTORS_EMITTED : RefCell < HashSet < String >> = RefCell :: default () ; } let ident = self . ident ; if ! DESCRIPTORS_EMITTED . with (| list | list . borrow_mut () . insert (ident . to_string ())) { return ; } let name = Ident :: new (& format ! ("__wbindgen_describe_{}" , ident) , ident . span ()) ; let inner = & self . inner ; let attrs = & self . attrs ; let wasm_bindgen = & self . wasm_bindgen ; (quote ! { # [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")))] # [automatically_derived] const _ : () = { # wasm_bindgen :: __wbindgen_coverage ! { # (# attrs) * # [no_mangle] # [doc (hidden)] pub extern "C" fn # name () { use # wasm_bindgen :: describe ::*; # wasm_bindgen :: __rt :: link_mem_intrinsics () ; # inner } } } ; }) . to_tokens (tokens) ; } }
};
}
