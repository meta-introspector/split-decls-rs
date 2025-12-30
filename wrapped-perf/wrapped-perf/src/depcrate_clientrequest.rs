// Generated macro for request (function)
macro_rules! Depcrate_clientrequest {
() => {
// Module: crate::client
// Provides: {"request"}
// Dependencies: {}
async fn request (mut send : quinn :: SendStream , mut upload : u64 , download : u64 , stream_stats : OpenStreamStats ,) -> Result < () > { let upload_start = Instant :: now () ; send . write_all (& download . to_be_bytes ()) . await ? ; if upload == 0 { send . finish () . unwrap () ; return Ok (()) ; } let send_stream_stats = stream_stats . new_sender (& send , upload) ; static DATA : [u8 ; 1024 * 1024] = [42 ; 1024 * 1024] ; while upload > 0 { let chunk_len = upload . min (DATA . len () as u64) ; send . write_chunk (Bytes :: from_static (& DATA [.. chunk_len as usize])) . await . context ("sending response") ? ; send_stream_stats . on_bytes (chunk_len as usize) ; upload -= chunk_len ; } send . finish () . unwrap () ; _ = send . stopped () . await ; send_stream_stats . finish (upload_start . elapsed ()) ; debug ! ("upload finished on {}" , send . id ()) ; Ok (()) }
};
}
