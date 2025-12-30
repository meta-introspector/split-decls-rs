// Generated macro for print_hash_and_path (function)
macro_rules! Depcrate_pack_receiveprint_hash_and_path {
() => {
// Module: crate::pack::receive
// Provides: {"print_hash_and_path"}
// Dependencies: {}
fn print_hash_and_path (out : & mut impl io :: Write , name : & str , id : ObjectId , path : Option < PathBuf >) -> io :: Result < () > { match path { Some (path) => writeln ! (out , "{}: {} ({})" , name , id , path . display ()) , None => writeln ! (out , "{name}: {id}") , } }
};
}
