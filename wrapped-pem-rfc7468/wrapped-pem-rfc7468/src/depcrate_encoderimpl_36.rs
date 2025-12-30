// Generated macro for impl_36 (impl)
macro_rules! Depcrate_encoderimpl_36 {
() => {
// Module: crate::encoder
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (feature = "std")] impl io :: Write for Encoder < '_ , '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . encode (buf) ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
