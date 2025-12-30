// Generated macro for impl_548 (impl)
macro_rules! Depcrate_enc_implsimpl_548 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_548"}
// Dependencies: {}
impl < T > Encode for RefCell < T > where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let borrow_guard = self . try_borrow () . map_err (| e | EncodeError :: RefCellAlreadyBorrowed { inner : e , type_name : core :: any :: type_name :: < RefCell < T > > () , }) ? ; T :: encode (& borrow_guard , encoder) } }
};
}
