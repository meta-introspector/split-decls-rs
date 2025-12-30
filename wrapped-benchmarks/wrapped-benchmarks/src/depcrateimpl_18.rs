// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a , C > Readable < 'a , C > for Signature where C : Context , { fn read_from < R : Reader < 'a , C > > (reader : & mut R) -> std :: result :: Result < Self , std :: io :: Error > { let mut data = [0u8 ; 32] ; reader . read_bytes (& mut data) ? ; Ok (Self (data)) } }
};
}
