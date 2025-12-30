// Generated macro for EnvelopeProxy (trait)
macro_rules! Depcrate_address_envelopeEnvelopeProxy {
() => {
// Module: crate::address::envelope
// Provides: {"EnvelopeProxy"}
// Dependencies: {}
pub trait EnvelopeProxy < A : Actor > { # [doc = " handle message within new actor and context"] fn handle (& mut self , act : & mut A , ctx : & mut A :: Context) ; }
};
}
