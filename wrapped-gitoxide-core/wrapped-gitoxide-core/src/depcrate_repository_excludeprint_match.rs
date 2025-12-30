// Generated macro for print_match (function)
macro_rules! Depcrate_repository_excludeprint_match {
() => {
// Module: crate::repository::exclude
// Provides: {"print_match"}
// Dependencies: {}
fn print_match (m : Option < gix :: ignore :: search :: Match < '_ > > , path : & BStr , mut out : impl std :: io :: Write ,) -> std :: io :: Result < () > { match m { Some (m) => writeln ! (out , "{}:{}:{}\t{}" , m . source . map (std :: path :: Path :: to_string_lossy) . unwrap_or_default () , m . sequence_number , m . pattern , path) , None => writeln ! (out , "::\t{path}") , } }
};
}
