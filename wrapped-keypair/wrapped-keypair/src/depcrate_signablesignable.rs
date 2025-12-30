// Generated macro for Signable (trait)
macro_rules! Depcrate_signableSignable {
() => {
// Module: crate::signable
// Provides: {"Signable"}
// Dependencies: {}
pub trait Signable { fn sign (& mut self , keypair : & Keypair) { let signature = keypair . sign_message (self . signable_data () . borrow ()) ; self . set_signature (signature) ; } fn verify (& self) -> bool { self . get_signature () . verify (self . pubkey () . as_ref () , self . signable_data () . borrow ()) } fn pubkey (& self) -> Address ; fn signable_data (& self) -> Cow < '_ , [u8] > ; fn get_signature (& self) -> Signature ; fn set_signature (& mut self , signature : Signature) ; }
};
}
