// Generated macro for input_iteration (module)
macro_rules! Depcrate_pack_createinput_iteration {
() => {
// Module: crate::pack::create
// Provides: {"input_iteration"}
// Dependencies: {}
pub mod input_iteration { use gix :: { hash , traverse } ; # [derive (Debug , thiserror :: Error)] pub enum Error { # [error ("input objects couldn't be iterated completely")] Iteration (# [from] traverse :: commit :: simple :: Error) , # [error ("An error occurred while reading hashes from standard input")] InputLinesIo (# [from] std :: io :: Error) , # [error ("Could not decode hex hash provided on standard input")] HashDecode (# [from] hash :: decode :: Error) , } }
};
}
