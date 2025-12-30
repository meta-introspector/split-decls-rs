// Generated macro for write_all (function)
macro_rules! Depcrate_stripwrite_all {
() => {
// Module: crate::strip
// Provides: {"write_all"}
// Dependencies: {}
fn write_all (raw : & mut dyn std :: io :: Write , state : & mut StripBytes , buf : & [u8] ,) -> std :: io :: Result < () > { for printable in state . strip_next (buf) { raw . write_all (printable) ? ; } Ok (()) }
};
}
