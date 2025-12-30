// Generated macro for send_data_on_stream (function)
macro_rules! Depcratesend_data_on_stream {
() => {
// Module: crate
// Provides: {"send_data_on_stream"}
// Dependencies: {}
pub async fn send_data_on_stream (stream : & mut quinn :: SendStream , stream_size : u64) -> Result < () > { const DATA : & [u8] = & [0xAB ; 1024 * 1024] ; let bytes_data = Bytes :: from_static (DATA) ; let full_chunks = stream_size / (DATA . len () as u64) ; let remaining = (stream_size % (DATA . len () as u64)) as usize ; for _ in 0 .. full_chunks { stream . write_chunk (bytes_data . clone ()) . await . context ("failed sending data") ? ; } if remaining != 0 { stream . write_chunk (bytes_data . slice (0 .. remaining)) . await . context ("failed sending data") ? ; } stream . finish () . unwrap () ; _ = stream . stopped () . await ; Ok (()) }
};
}
