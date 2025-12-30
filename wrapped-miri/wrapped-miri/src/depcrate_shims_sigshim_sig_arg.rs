// Generated macro for shim_sig_arg (macro)
macro_rules! Depcrate_shims_sigshim_sig_arg {
() => {
// Module: crate::shims::sig
// Provides: {"shim_sig_arg"}
// Dependencies: {}
# [doc = " Helper for `shim_sig!`."] # [macro_export] macro_rules ! shim_sig_arg { ($ this : ident , $ x : ty) => { { match stringify ! ($ x) { "i8" => $ this . tcx . types . i8 , "i16" => $ this . tcx . types . i16 , "i32" => $ this . tcx . types . i32 , "i64" => $ this . tcx . types . i64 , "i128" => $ this . tcx . types . i128 , "isize" => $ this . tcx . types . isize , "u8" => $ this . tcx . types . u8 , "u16" => $ this . tcx . types . u16 , "u32" => $ this . tcx . types . u32 , "u64" => $ this . tcx . types . u64 , "u128" => $ this . tcx . types . u128 , "usize" => $ this . tcx . types . usize , "()" => $ this . tcx . types . unit , "*const _" => $ this . machine . layouts . const_raw_ptr . ty , "*mut _" => $ this . machine . layouts . mut_raw_ptr . ty , ty if let Some (libc_ty) = ty . strip_prefix ("libc::") => $ this . libc_ty_layout (libc_ty) . ty , ty => panic ! ("unsupported signature type {ty:?}") , } } } ; }
};
}
