// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl std :: fmt :: Display for HexSlice < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { for byte in self . 0 { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}
