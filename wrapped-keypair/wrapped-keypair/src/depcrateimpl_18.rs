// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl EncodableKey for Keypair { fn read < R : Read > (reader : & mut R) -> Result < Self , Box < dyn error :: Error > > { read_keypair (reader) } fn write < W : Write > (& self , writer : & mut W) -> Result < String , Box < dyn error :: Error > > { write_keypair (self , writer) } }
};
}
