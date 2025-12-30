// Generated macro for nonce_seq_key_op_mut (macro)
macro_rules! Depcrate_aeadnonce_seq_key_op_mut {
() => {
// Module: crate::aead
// Provides: {"nonce_seq_key_op_mut"}
// Dependencies: {}
macro_rules ! nonce_seq_key_op_mut { ($ name : ident , $ name_prep_nonce : ident) => { # [doc = " A key operation with a precomputed nonce from a key's associated `NonceSequence`."] pub struct $ name_prep_nonce <'a , N : NonceSequence > { key : &'a mut $ name < N >, nonce : Nonce , } impl <'a , N : NonceSequence > $ name_prep_nonce <'a , N > { fn new (key : &'a mut $ name < N >) -> Result < Self , Unspecified > { let nonce = key . nonce_sequence . advance () ?; Ok (Self { key , nonce }) } } impl < N : NonceSequence > $ name_prep_nonce <'_ , N > { # [doc = " Returns the prepared Nonce that is used for key methods invoked on [Self]."] # [must_use] pub fn nonce (& self) -> & Nonce { & self . nonce } } impl < N : NonceSequence > Debug for $ name_prep_nonce <'_ , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter <'_ >) -> Result < () , core :: fmt :: Error > { f . debug_struct (stringify ! ($ name_prep_nonce)) . finish_non_exhaustive () } } } ; }
};
}
