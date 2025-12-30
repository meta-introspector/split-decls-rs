// Generated macro for impl_40 (impl)
macro_rules! Depcrate_store_impls_dynamic_headerimpl_40 {
() => {
// Module: crate::store_impls::dynamic::header
// Provides: {"impl_40"}
// Dependencies: {}
impl < S > crate :: Header for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { fn try_header (& self , id : & oid) -> Result < Option < Header > , gix_object :: find :: Error > { let mut snapshot = self . snapshot . borrow_mut () ; let mut inflate = self . inflate . borrow_mut () ; self . try_header_inner (id , & mut inflate , & mut snapshot , None) . map_err (| err | Box :: new (err) as _) } }
};
}
