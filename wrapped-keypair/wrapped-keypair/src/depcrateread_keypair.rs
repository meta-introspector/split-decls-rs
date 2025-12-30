// Generated macro for read_keypair (function)
macro_rules! Depcrateread_keypair {
() => {
// Module: crate
// Provides: {"read_keypair"}
// Dependencies: {}
# [doc = " Reads a JSON-encoded `Keypair` from a `Reader` implementor"] pub fn read_keypair < R : Read > (reader : & mut R) -> Result < Keypair , Box < dyn error :: Error > > { let mut buffer = String :: new () ; reader . read_to_string (& mut buffer) ? ; let trimmed = buffer . trim () ; if ! trimmed . starts_with ('[') || ! trimmed . ends_with (']') { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidData , "Input must be a JSON array" ,) . into ()) ; } # [allow (clippy :: arithmetic_side_effects)] let contents = & trimmed [1 .. trimmed . len () - 1] ; let elements_vec : Vec < & str > = contents . split (',') . map (| s | s . trim ()) . collect () ; let len = elements_vec . len () ; let elements : [& str ; ed25519_dalek :: KEYPAIR_LENGTH] = elements_vec . try_into () . map_err (| _ | { std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidData , format ! ("Expected {} elements, found {}" , ed25519_dalek :: KEYPAIR_LENGTH , len) ,) }) ? ; let mut out = [0u8 ; ed25519_dalek :: KEYPAIR_LENGTH] ; for (idx , element) in elements . into_iter () . enumerate () { let parsed : u8 = element . parse () ? ; out [idx] = parsed ; } Keypair :: try_from (& out [..]) . map_err (| e | std :: io :: Error :: other (e . to_string ()) . into ()) }
};
}
