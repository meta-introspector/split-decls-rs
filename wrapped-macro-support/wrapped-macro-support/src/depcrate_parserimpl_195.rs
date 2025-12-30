// Generated macro for impl_195 (impl)
macro_rules! Depcrate_parserimpl_195 {
() => {
// Module: crate::parser
// Provides: {"impl_195"}
// Dependencies: {}
impl < 'a > ConvertToAst < (& ast :: Program , BindgenAttrs , & 'a Option < ast :: ImportModule >) > for syn :: ItemStatic { type Target = ast :: ImportKind ; fn convert (self , (program , opts , module) : (& ast :: Program , BindgenAttrs , & 'a Option < ast :: ImportModule >) ,) -> Result < Self :: Target , Diagnostic > { if let syn :: StaticMutability :: Mut (_) = self . mutability { bail_span ! (self . mutability , "cannot import mutable globals yet") } let string = if let syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (string) , .. }) = * self . expr . clone () { string . value () } else { bail_span ! (self . expr , "statics with a value can only be string literals") } ; if opts . static_string () . is_none () { bail_span ! (self , "static strings require `#[wasm_bindgen(static_string)]`") } let thread_local = if let Some (thread_local) = opts . get_thread_local () ? { thread_local } else { bail_span ! (self , "static strings require `#[wasm_bindgen(thread_local_v2)]`") } ; let shim = format ! ("__wbg_string_{}_{}" , self . ident , ShortHash ((& module , & self . ident)) ,) ; opts . check_used () ; Ok (ast :: ImportKind :: String (ast :: ImportString { ty : * self . ty , vis : self . vis , rust_name : self . ident . clone () , shim : Ident :: new (& shim , Span :: call_site ()) , wasm_bindgen : program . wasm_bindgen . clone () , js_sys : program . js_sys . clone () , string , thread_local , })) } }
};
}
