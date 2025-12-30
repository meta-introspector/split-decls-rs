// Generated macro for impl_23 (impl)
macro_rules! Depcrate_object_idimpl_23 {
() => {
// Module: crate::object_id
// Provides: {"impl_23"}
// Dependencies: {}
impl std :: fmt :: Debug for ObjectId { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { ObjectId :: Sha1 (_hash) => f . write_str ("Sha1(") ? , } for b in self . as_bytes () { write ! (f , "{b:02x}") ? ; } f . write_str (")") } }
};
}
