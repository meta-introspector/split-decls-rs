// Generated macro for read_str_data (function)
macro_rules! Depcrate_decoderead_str_data {
() => {
// Module: crate::decode
// Provides: {"read_str_data"}
// Dependencies: {}
fn read_str_data < 'de , V , R > (rd : & mut R , len : u32 , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , R : ReadSlice < 'de > { match read_bin_data (rd , len) ? { Reference :: Borrowed (buf) => { match str :: from_utf8 (buf) { Ok (s) => visitor . visit_borrowed_str (s) , Err (err) => { match visitor . visit_borrowed_bytes :: < Error > (buf) { Ok (buf) => Ok (buf) , Err (..) => Err (Error :: Utf8Error (err)) , } } } } Reference :: Copied (buf) => { match str :: from_utf8 (buf) { Ok (s) => visitor . visit_str (s) , Err (err) => { match visitor . visit_bytes :: < Error > (buf) { Ok (buf) => Ok (buf) , Err (..) => Err (Error :: Utf8Error (err)) , } } } } } }
};
}
