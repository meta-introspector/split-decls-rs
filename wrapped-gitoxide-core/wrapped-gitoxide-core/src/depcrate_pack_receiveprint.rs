// Generated macro for print (function)
macro_rules! Depcrate_pack_receiveprint {
() => {
// Module: crate::pack::receive
// Provides: {"print"}
// Dependencies: {}
fn print (out : & mut impl io :: Write , res : pack :: bundle :: write :: Outcome , refs : & [Ref]) -> io :: Result < () > { print_hash_and_path (out , "index" , res . index . index_hash , res . index_path) ? ; print_hash_and_path (out , "pack" , res . index . data_hash , res . data_path) ? ; writeln ! (out) ? ; crate :: repository :: remote :: refs :: print (out , refs) ? ; Ok (()) }
};
}
