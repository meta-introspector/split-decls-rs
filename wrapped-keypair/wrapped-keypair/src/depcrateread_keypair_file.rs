// Generated macro for read_keypair_file (function)
macro_rules! Depcrateread_keypair_file {
() => {
// Module: crate
// Provides: {"read_keypair_file"}
// Dependencies: {}
# [doc = " Reads a `Keypair` from a file"] pub fn read_keypair_file < F : AsRef < Path > > (path : F) -> Result < Keypair , Box < dyn error :: Error > > { Keypair :: read_from_file (path) }
};
}
