// Generated macro for api_tags (module)
macro_rules! Depcrate_bridgeapi_tags {
() => {
// Module: crate::bridge
// Provides: {"api_tags"}
// Dependencies: {}
# [forbid (unsafe_code)] # [allow (non_camel_case_types)] mod api_tags { use super :: rpc :: { DecodeMut , Encode , Reader , Writer } ; macro_rules ! declare_tags { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) *;) * }) ,* $ (,) ?) => { $ (pub (super) enum $ name { $ ($ method) ,* } rpc_encode_decode ! (enum $ name { $ ($ method) ,* }) ;) * pub (super) enum Method { $ ($ name ($ name)) ,* } rpc_encode_decode ! (enum Method { $ ($ name (m)) ,* }) ; } } with_api ! (self , self , declare_tags) ; }
};
}
