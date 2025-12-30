// Generated macro for hash_path (function)
macro_rules! Depcratehash_path {
() => {
// Module: crate
// Provides: {"hash_path"}
// Dependencies: {}
fn hash_path (args : & Args , path : & Path) -> anyhow :: Result < blake3 :: OutputReader > { let mut hasher = args . base_hasher . clone () ; if path == Path :: new ("-") { if args . keyed () { bail ! ("Cannot open `-` in keyed mode") ; } hasher . update_reader (io :: stdin () . lock ()) ? ; } else if args . no_mmap () { hasher . update_reader (File :: open (path) ?) ? ; } else { hasher . update_mmap_rayon (path) ? ; } let mut output_reader = hasher . finalize_xof () ; output_reader . set_position (args . seek ()) ; Ok (output_reader) }
};
}
