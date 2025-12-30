// Generated macro for wasm_filesize (function)
macro_rules! Depcratewasm_filesize {
() => {
// Module: crate
// Provides: {"wasm_filesize"}
// Dependencies: {}
fn wasm_filesize (dir : & str , filesuffix : & str) -> Result < u64 , std :: io :: Error > { let paths = fs :: read_dir (dir) . expect ("Directory with wasm binaries not found!") ; let mut count : u64 = 0 ; for path in paths { let p = path . unwrap () . path () ; if let Some (suffix) = p . extension () { if suffix == filesuffix { count += 1 ; println ! ("{{\"biggerIsBetter\":false,\"name\":{:?},\"unit\":\"bytes\",\"value\":{}}}" , p . file_name () . unwrap () , p . metadata () ?. len ()) ; } } } Ok (count) }
};
}
