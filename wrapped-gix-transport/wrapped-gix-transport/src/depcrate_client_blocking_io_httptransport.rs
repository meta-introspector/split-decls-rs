// Generated macro for Transport (struct)
macro_rules! Depcrate_client_blocking_io_httpTransport {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"Transport"}
// Dependencies: {}
# [doc = " A transport for supporting arbitrary http clients by abstracting interactions with them into the [Http] trait."] pub struct Transport < H : Http > { url : String , user_agent_header : & 'static str , desired_version : Protocol , actual_version : Protocol , http : H , service : Option < Service > , line_provider : Option < StreamingPeekableIter < H :: ResponseBody > > , identity : Option < gix_sec :: identity :: Account > , trace : bool , }
};
}
