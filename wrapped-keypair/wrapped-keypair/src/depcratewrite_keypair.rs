// Generated macro for write_keypair (function)
macro_rules! Depcratewrite_keypair {
() => {
// Module: crate
// Provides: {"write_keypair"}
// Dependencies: {}
# [doc = " Writes a `Keypair` to a `Write` implementor with JSON-encoding"] pub fn write_keypair < W : Write > (keypair : & Keypair , writer : & mut W ,) -> Result < String , Box < dyn error :: Error > > { let keypair_bytes = keypair . to_bytes () ; let mut result = Vec :: with_capacity (64 * 4 + 2) ; result . push (b'[') ; for (i , & num) in keypair_bytes . iter () . enumerate () { if i > 0 { result . push (b',') ; } let num_str = num . to_string () ; result . extend_from_slice (num_str . as_bytes ()) ; } result . push (b']') ; writer . write_all (& result) ? ; let as_string = String :: from_utf8 (result) ? ; Ok (as_string) }
};
}
