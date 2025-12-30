// Generated macro for singlebyte (macro)
macro_rules! Depcrate_allsinglebyte {
() => {
// Module: crate::all
// Provides: {"singlebyte"}
// Dependencies: {}
macro_rules ! singlebyte (($ (# [$ attr : meta]) * var =$ var : ident , mod =$ ($ module : ident) ::+, name =$ name : expr) => (singlebyte ! ($ (# [$ attr]) * var =$ var , mod =$ ($ module) ::+, name =$ name , whatwg = None) ;) ; ($ (# [$ attr : meta]) * var =$ var : ident , mod =$ ($ module : ident) ::+, name | whatwg =$ name : expr) => (singlebyte ! ($ (# [$ attr]) * var =$ var , mod =$ ($ module) ::+, name =$ name , whatwg = Some ($ name)) ;) ; ($ (# [$ attr : meta]) * var =$ var : ident , mod =$ ($ module : ident) ::+, name =$ name : expr , whatwg =$ whatwg : expr) => ($ (# [$ attr]) * pub const $ var : &'static codec :: singlebyte :: SingleByteEncoding = & codec :: singlebyte :: SingleByteEncoding { name : $ name , whatwg_name : $ whatwg , index_forward : $ ($ module) ::+:: forward , index_backward : $ ($ module) ::+:: backward , } ;)) ;
};
}
