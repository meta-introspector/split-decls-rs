// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl fmt :: Debug for AccountInfo < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("AccountInfo") ; f . field ("key" , & self . key) . field ("owner" , & self . owner) . field ("is_signer" , & self . is_signer) . field ("is_writable" , & self . is_writable) . field ("executable" , & self . executable) . field ("lamports" , & self . lamports ()) . field ("data.len" , & self . data_len ()) ; debug_account_data :: debug_account_data (& self . data . borrow () , & mut f) ; f . finish_non_exhaustive () } }
};
}
