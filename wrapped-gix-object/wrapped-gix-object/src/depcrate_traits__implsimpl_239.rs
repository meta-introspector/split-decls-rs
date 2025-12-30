// Generated macro for impl_239 (impl)
macro_rules! Depcrate_traits__implsimpl_239 {
() => {
// Module: crate::traits::_impls
// Provides: {"impl_239"}
// Dependencies: {}
impl < T > crate :: Write for Arc < T > where T : crate :: Write , { fn write (& self , object : & dyn WriteTo) -> Result < ObjectId , crate :: write :: Error > { self . deref () . write (object) } fn write_buf (& self , object : Kind , from : & [u8]) -> Result < ObjectId , crate :: write :: Error > { self . deref () . write_buf (object , from) } fn write_stream (& self , kind : Kind , size : u64 , from : & mut dyn Read) -> Result < ObjectId , crate :: write :: Error > { self . deref () . write_stream (kind , size , from) } }
};
}
