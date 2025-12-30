// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < C : Context > Writable < C > for CryptoHash { fn write_to < 'a , T : ? Sized + Writer < 'a , C > > (& 'a self , writer : & mut T ,) -> std :: result :: Result < () , std :: io :: Error > { writer . write_bytes (& self . 0) . map (| _ | ()) } }
};
}
