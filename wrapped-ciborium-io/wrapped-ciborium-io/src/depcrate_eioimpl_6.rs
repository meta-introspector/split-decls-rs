// Generated macro for impl_6 (impl)
macro_rules! Depcrate_eioimpl_6 {
() => {
// Module: crate::eio
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a , W > Write for EIOWriter < 'a , W > where W : embedded_io :: Write , { type Error = W :: Error ; fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > { embedded_io :: Write :: write_all (self . 0 , data) } fn flush (& mut self) -> Result < () , Self :: Error > { embedded_io :: Write :: flush (self . 0) } }
};
}
