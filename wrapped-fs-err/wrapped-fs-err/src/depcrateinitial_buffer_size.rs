// Generated macro for initial_buffer_size (function)
macro_rules! Depcrateinitial_buffer_size {
() => {
// Module: crate
// Provides: {"initial_buffer_size"}
// Dependencies: {}
fn initial_buffer_size (file : & std :: fs :: File) -> usize { file . metadata () . map (| m | m . len () as usize + 1) . unwrap_or (0) }
};
}
