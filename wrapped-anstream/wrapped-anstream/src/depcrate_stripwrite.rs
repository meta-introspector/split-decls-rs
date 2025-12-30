// Generated macro for write (function)
macro_rules! Depcrate_stripwrite {
() => {
// Module: crate::strip
// Provides: {"write"}
// Dependencies: {}
fn write (raw : & mut dyn std :: io :: Write , state : & mut StripBytes , buf : & [u8] ,) -> std :: io :: Result < usize > { let initial_state = state . clone () ; for printable in state . strip_next (buf) { let possible = printable . len () ; let written = raw . write (printable) ? ; if possible != written { let divergence = & printable [written ..] ; let offset = offset_to (buf , divergence) ; let consumed = & buf [offset ..] ; * state = initial_state ; state . strip_next (consumed) . last () ; return Ok (offset) ; } } Ok (buf . len ()) }
};
}
