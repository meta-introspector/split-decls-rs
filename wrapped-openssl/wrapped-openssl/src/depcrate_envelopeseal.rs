// Generated macro for Seal (struct)
macro_rules! Depcrate_envelopeSeal {
() => {
// Module: crate::envelope
// Provides: {"Seal"}
// Dependencies: {}
# [doc = " Represents an EVP_Seal context."] pub struct Seal { ctx : CipherCtx , iv : Option < Vec < u8 > > , enc_keys : Vec < Vec < u8 > > , }
};
}
