// Generated macro for impl_9 (impl)
macro_rules! Depcrate_eioimpl_9 {
() => {
// Module: crate::eio
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , R > Read for EIOReader < 'a , R > where R : embedded_io :: Read , { type Error = embedded_io :: ReadExactError < R :: Error > ; fn read_exact (& mut self , data : & mut [u8]) -> Result < () , Self :: Error > { embedded_io :: Read :: read_exact (self . 0 , data) } }
};
}
