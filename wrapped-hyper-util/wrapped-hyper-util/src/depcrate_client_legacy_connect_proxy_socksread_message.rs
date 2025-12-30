// Generated macro for read_message (function)
macro_rules! Depcrate_client_legacy_connect_proxy_socksread_message {
() => {
// Module: crate::client::legacy::connect::proxy::socks
// Provides: {"read_message"}
// Dependencies: {}
async fn read_message < T , M , C > (mut conn : & mut T , buf : & mut BytesMut) -> Result < M , SocksError < C > > where T : Read + Unpin , M : for < 'a > TryFrom < & 'a mut BytesMut , Error = ParsingError > , { let mut tmp = [0 ; 513] ; loop { let n = crate :: rt :: read (& mut conn , & mut tmp) . await ? ; buf . extend_from_slice (& tmp [.. n]) ; match M :: try_from (buf) { Err (ParsingError :: Incomplete) => { if n == 0 { if buf . spare_capacity_mut () . is_empty () { return Err (SocksError :: Parsing (ParsingError :: WouldOverflow)) ; } else { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: UnexpectedEof , "unexpected eof" ,) . into ()) ; } } } Err (err) => return Err (err . into ()) , Ok (res) => return Ok (res) , } } }
};
}
