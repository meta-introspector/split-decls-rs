// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let data = vec ! [0u8 ; BYTES] ; run_benchmark_group (| group | { group . register_benchmark ("bufreader_snappy" , | | { | | { let mut compressed = Vec :: new () ; FrameEncoder :: new (& mut compressed) . write_all (data . as_slice ()) . unwrap () ; let mut reader = BufReader :: with_capacity (BYTES , FrameDecoder :: new (& compressed [..])) ; while let Ok (buf) = reader . fill_buf () { if buf . is_empty () { break ; } black_box (buf) ; let len = buf . len () ; reader . consume (len) ; } compressed } }) ; }) ; }
};
}
