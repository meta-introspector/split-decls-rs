// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let filename = std :: env :: args () . nth (1) . expect ("filename") ; let mode = std :: env :: args () . nth (2) ; let mode = mode . as_deref () . unwrap_or ("rust-oneshot") ; let file = std :: fs :: read (filename) . expect ("read") ; let chunk_size = file . len () / 100 ; let chunk_size = usize :: max (chunk_size , 1) ; let start = Instant :: now () ; let hash = match mode { "rust-oneshot" => rust_oneshot (& file) , "c-oneshot" => c_oneshot (& file) , "rust-chunked" => rust_chunked (& file , chunk_size) , "c-chunked" => c_chunked (& file , chunk_size) , other => panic ! ("Unknown mode {other}") , } ; let elapsed = start . elapsed () ; eprintln ! ("{mode}\t{elapsed:?}\t{hash:016X}") ; }
};
}
