// Generated macro for entries (function)
macro_rules! Depcrate_writeentries {
() => {
// Module: crate::write
// Provides: {"entries"}
// Dependencies: {}
fn entries < T : std :: io :: Write > (out : & mut CountBytes < T > , state : & State , header_size : u32) -> Result < u32 , std :: io :: Error > { for entry in state . entries () { if entry . flags . contains (entry :: Flags :: REMOVE) { continue ; } entry . write_to (& mut * out , state) ? ; match (out . count - header_size) % 8 { 0 => { } n => { let eight_null_bytes = [0u8 ; 8] ; out . write_all (& eight_null_bytes [n as usize ..]) ? ; } } } Ok (out . count) }
};
}
