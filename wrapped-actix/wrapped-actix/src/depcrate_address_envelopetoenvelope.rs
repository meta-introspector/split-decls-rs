// Generated macro for ToEnvelope (trait)
macro_rules! Depcrate_address_envelopeToEnvelope {
() => {
// Module: crate::address::envelope
// Provides: {"ToEnvelope"}
// Dependencies: {}
# [doc = " Converter trait, packs message into a suitable envelope."] pub trait ToEnvelope < A , M : Message > where A : Actor + Handler < M > , A :: Context : ToEnvelope < A , M > , { # [doc = " Pack message into suitable envelope"] fn pack (msg : M , tx : Option < Sender < M :: Result > >) -> Envelope < A > ; }
};
}
