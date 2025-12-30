// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < T > PartialEq < T > for Keypair where T : Signer , { fn eq (& self , other : & T) -> bool { self . pubkey () == other . pubkey () } }
};
}
