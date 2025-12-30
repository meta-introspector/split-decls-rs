// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < C : Context > Writable < C > for Signature { fn write_to < 'a , T : ? Sized + Writer < 'a , C > > (& 'a self , writer : & mut T ,) -> std :: result :: Result < () , std :: io :: Error > { writer . write_bytes (& self . 0) . map (| _ | ()) } }
};
}
