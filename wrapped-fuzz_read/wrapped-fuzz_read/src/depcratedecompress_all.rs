// Generated macro for decompress_all (function)
macro_rules! Depcratedecompress_all {
() => {
// Module: crate
// Provides: {"decompress_all"}
// Dependencies: {}
fn decompress_all (data : & [u8]) -> Result < () , Box < dyn std :: error :: Error > > { let reader = std :: io :: Cursor :: new (data) ; let mut zip = zip :: ZipArchive :: new (reader) ? ; for i in 0 .. zip . len () { let mut file = zip . by_index (i) ? . take (MAX_BYTES_TO_READ) ; std :: io :: copy (& mut file , & mut std :: io :: sink ()) ? ; } let mut reader = zip . into_inner () ; reader . seek (SeekFrom :: Start (0)) ? ; while let Ok (Some (mut file)) = read_zipfile_from_stream (& mut reader) { std :: io :: copy (& mut file , & mut std :: io :: sink ()) ? ; } Ok (()) }
};
}
