// Generated macro for read_line_internal (function)
macro_rules! Depcrate_ioread_line_internal {
() => {
// Module: crate::io
// Provides: {"read_line_internal"}
// Dependencies: {}
fn read_line_internal < R : AsyncBufRead + ? Sized > (reader : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut String , bytes : & mut Vec < u8 > , read : & mut usize ,) -> Poll < Result < usize > > { let ret = ready ! (read_until_internal (reader , cx , b'\n' , bytes , read)) ; match String :: from_utf8 (mem :: take (bytes)) { Ok (s) => { debug_assert ! (buf . is_empty ()) ; debug_assert_eq ! (* read , 0) ; * buf = s ; Poll :: Ready (ret) } Err (_) => Poll :: Ready (ret . and_then (| _ | { Err (Error :: new (ErrorKind :: InvalidData , "stream did not contain valid UTF-8" ,)) })) , } }
};
}
