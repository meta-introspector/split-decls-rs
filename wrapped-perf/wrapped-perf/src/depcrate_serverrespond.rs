// Generated macro for respond (function)
macro_rules! Depcrate_serverrespond {
() => {
// Module: crate::server
// Provides: {"respond"}
// Dependencies: {}
async fn respond (mut bytes : u64 , mut stream : quinn :: SendStream) -> Result < () > { static DATA : [u8 ; 1024 * 1024] = [42 ; 1024 * 1024] ; while bytes > 0 { let chunk_len = bytes . min (DATA . len () as u64) ; stream . write_chunk (Bytes :: from_static (& DATA [.. chunk_len as usize])) . await . context ("sending response") ? ; bytes -= chunk_len ; } debug ! ("finished responding on {}" , stream . id ()) ; Ok (()) }
};
}
