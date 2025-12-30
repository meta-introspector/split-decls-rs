// Generated macro for impl_476 (impl)
macro_rules! Depcrate_hashimpl_476 {
() => {
// Module: crate::hash
// Provides: {"impl_476"}
// Dependencies: {}
impl Clone for Hasher { fn clone (& self) -> Hasher { let ctx = unsafe { let ctx = EVP_MD_CTX_new () ; assert ! (! ctx . is_null ()) ; let r = ffi :: EVP_MD_CTX_copy_ex (ctx , self . ctx) ; assert_eq ! (r , 1) ; ctx } ; Hasher { ctx , md : self . md , type_ : self . type_ , state : self . state , } } }
};
}
