// Generated macro for impl_38 (impl)
macro_rules! Depcrate_encoderimpl_38 {
() => {
// Module: crate::encoder
// Provides: {"impl_38"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : Encoding > io :: Write for Encoder < '_ , E > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . encode (buf) ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
