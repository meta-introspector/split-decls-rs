// Generated macro for write_keypair_file (function)
macro_rules! Depcratewrite_keypair_file {
() => {
// Module: crate
// Provides: {"write_keypair_file"}
// Dependencies: {}
# [doc = " Writes a `Keypair` to a file with JSON-encoding"] pub fn write_keypair_file < F : AsRef < Path > > (keypair : & Keypair , outfile : F ,) -> Result < String , Box < dyn error :: Error > > { keypair . write_to_file (outfile) }
};
}
