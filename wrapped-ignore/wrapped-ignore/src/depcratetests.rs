// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: { env , fs , path :: { Path , PathBuf } , } ; # [doc = " A convenient result type alias."] pub (crate) type Result < T > = std :: result :: Result < T , Box < dyn std :: error :: Error + Send + Sync > > ; macro_rules ! err { ($ ($ tt : tt) *) => { Box ::< dyn std :: error :: Error + Send + Sync >:: from (format ! ($ ($ tt) *)) } } # [doc = " A simple wrapper for creating a temporary directory that is"] # [doc = " automatically deleted when it's dropped."] # [doc = ""] # [doc = " We use this in lieu of tempfile because tempfile brings in too many"] # [doc = " dependencies."] # [derive (Debug)] pub struct TempDir (PathBuf) ; impl Drop for TempDir { fn drop (& mut self) { fs :: remove_dir_all (& self . 0) . unwrap () ; } } impl TempDir { # [doc = " Create a new empty temporary directory under the system's configured"] # [doc = " temporary directory."] pub fn new () -> Result < TempDir > { use std :: sync :: atomic :: { AtomicUsize , Ordering } ; static TRIES : usize = 100 ; static COUNTER : AtomicUsize = AtomicUsize :: new (0) ; let tmpdir = env :: temp_dir () ; for _ in 0 .. TRIES { let count = COUNTER . fetch_add (1 , Ordering :: Relaxed) ; let path = tmpdir . join ("rust-ignore") . join (count . to_string ()) ; if path . is_dir () { continue ; } fs :: create_dir_all (& path) . map_err (| e | { err ! ("failed to create {}: {}" , path . display () , e) }) ? ; return Ok (TempDir (path)) ; } Err (err ! ("failed to create temp dir after {} tries" , TRIES)) } # [doc = " Return the underlying path to this temporary directory."] pub fn path (& self) -> & Path { & self . 0 } } }
};
}
