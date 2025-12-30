// Generated macro for impl_97 (impl)
macro_rules! Depcrateimpl_97 {
() => {
// Module: crate
// Provides: {"impl_97"}
// Dependencies: {}
impl UnknownTransportParameters { # [doc = " Pushes an unknown transport parameter into storage if there is space"] # [doc = " remaining."] pub fn push (& mut self , new : UnknownTransportParameter < & [u8] >) -> Result < () > { let new_unknown_tp_size = new . value . len () + size_of :: < u64 > () ; if new_unknown_tp_size < self . capacity { self . capacity -= new_unknown_tp_size ; self . parameters . push (new . into ()) ; Ok (()) } else { Err (octets :: BufferTooShortError . into ()) } } }
};
}
