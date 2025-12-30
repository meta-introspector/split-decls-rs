// Generated macro for compress (function)
macro_rules! Depcratecompress {
() => {
// Module: crate
// Provides: {"compress"}
// Dependencies: {}
fn compress (input : PathBuf , output : PathBuf , level : u8) -> color_eyre :: Result < () > { info ! ("compressing {input:?} to {output:?}") ; let compression_level : ruzstd :: encoding :: CompressionLevel = match level { 0 => CompressionLevel :: Uncompressed , 1 => CompressionLevel :: Fastest , 2 => CompressionLevel :: Default , 3 => CompressionLevel :: Better , 4 => CompressionLevel :: Best , _ => { unimplemented ! ("unsupported compression level: {}" , level) ; } } ; let source_file = File :: open (input) . wrap_err ("failed to open input file") ? ; let source_size = source_file . metadata () ? . len () as usize ; let buffered_source = BufReader :: new (source_file) ; let encoder_input = ProgressMonitor :: new (buffered_source , source_size) ; let output : File = File :: create (output) . wrap_err ("failed to open output file for writing") ? ; ruzstd :: encoding :: compress (encoder_input , & output , compression_level) ; let compressed_size = output . metadata () ? . len () ; let compression_ratio = compressed_size as f64 / source_size as f64 * 100.0 ; info ! ("{} ——> {} ({compression_ratio:.2}%)" , fmt_size (source_size as f64) , fmt_size (compressed_size as f64)) ; Ok (()) }
};
}
