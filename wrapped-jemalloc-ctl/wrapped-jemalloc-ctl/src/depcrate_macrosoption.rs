// Generated macro for option (macro)
macro_rules! Depcrate_macrosoption {
() => {
// Module: crate::macros
// Provides: {"option"}
// Dependencies: {}
# [doc = " Creates a new option"] macro_rules ! option { ($ id : ident [str : $ byte_string : expr , $ mib : ty , $ name_to_mib : ident] => $ ret_ty : ty | ops : $ ($ ops : ident) ,* | docs : $ (# [$ doc : meta]) * mib_docs : $ (# [$ doc_mib : meta]) *) => { types ! { $ id [str : $ byte_string , $ mib , $ name_to_mib] | docs : $ (# [$ doc]) * mib_docs : $ (# [$ doc_mib]) * } $ ($ ops ! ($ id => $ ret_ty) ;) * } ; ($ id : ident [str : $ byte_string : expr , non_str : $ mib_len : expr] => $ ret_ty : ty | ops : $ ($ ops : ident) ,* | docs : $ (# [$ doc : meta]) * mib_docs : $ (# [$ doc_mib : meta]) *) => { option ! { $ id [str : $ byte_string , Mib < [usize ; $ mib_len] >, mib] => $ ret_ty | ops : $ ($ ops) ,* | docs : $ (# [$ doc]) * mib_docs : $ (# [$ doc_mib]) * } } ; ($ id : ident [str : $ byte_string : expr , str : $ mib_len : expr] => $ ret_ty : ty | ops : $ ($ ops : ident) ,* | docs : $ (# [$ doc : meta]) * mib_docs : $ (# [$ doc_mib : meta]) *) => { option ! { $ id [str : $ byte_string , MibStr < [usize ; $ mib_len] >, mib_str] => $ ret_ty | ops : $ ($ ops) ,* | docs : $ (# [$ doc]) * mib_docs : $ (# [$ doc_mib]) * } } ; }
};
}
