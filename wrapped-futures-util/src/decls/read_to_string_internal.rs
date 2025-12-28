macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! read_to_string_internal {
    () => {
        deps!();
        fn read_to_string_internal < R : AsyncRead + ? Sized > (reader : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut String , bytes : & mut Vec < u8 > , start_len : usize ,) -> Poll < io :: Result < usize > > { let ret = ready ! (read_to_end_internal (reader , cx , bytes , start_len)) ; if str :: from_utf8 (bytes) . is_err () { Poll :: Ready (ret . and_then (| _ | { Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "stream did not contain valid UTF-8")) })) } else { debug_assert ! (buf . is_empty ()) ; mem :: swap (unsafe { buf . as_mut_vec () } , bytes) ; Poll :: Ready (ret) } }
    };
}

read_to_string_internal!();