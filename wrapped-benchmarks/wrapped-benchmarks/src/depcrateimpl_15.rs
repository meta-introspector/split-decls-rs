// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < C : Context > Writable < C > for MerkleHash { fn write_to < 'a , T : ? Sized + Writer < 'a , C > > (& 'a self , writer : & mut T ,) -> std :: result :: Result < () , std :: io :: Error > { writer . write_bytes (& self . 0) . map (| _ | ()) } }
};
}
