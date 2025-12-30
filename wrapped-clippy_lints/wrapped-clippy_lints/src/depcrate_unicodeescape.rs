// Generated macro for escape (function)
macro_rules! Depcrate_unicodeescape {
() => {
// Module: crate::unicode
// Provides: {"escape"}
// Dependencies: {}
fn escape < T : Iterator < Item = char > > (s : T) -> String { let mut result = String :: new () ; for c in s { if c as u32 > 0x7F { for d in c . escape_unicode () { result . push (d) ; } } else { result . push (c) ; } } result }
};
}
