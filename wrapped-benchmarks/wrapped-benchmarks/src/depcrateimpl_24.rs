// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < C : Context > Writable < C > for PublicKey { fn write_to < 'a , T : ? Sized + Writer < 'a , C > > (& 'a self , writer : & mut T ,) -> std :: result :: Result < () , std :: io :: Error > { writer . write_bytes (& self . 0) . map (| _ | ()) } }
};
}
