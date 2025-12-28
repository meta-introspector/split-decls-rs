macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! read_line_internal {
    () => {
        deps!();
        pub (super) fn read_line_internal < R : AsyncBufRead + ? Sized > (reader : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut String , bytes : & mut Vec < u8 > , read : & mut usize ,) -> Poll < io :: Result < usize > > { let mut ret = ready ! (read_until_internal (reader , cx , b'\n' , bytes , read)) ; if str :: from_utf8 (& bytes [bytes . len () - * read .. bytes . len ()]) . is_err () { bytes . truncate (bytes . len () - * read) ; if ret . is_ok () { ret = Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "stream did not contain valid UTF-8" ,)) ; } } * read = 0 ; mem :: swap (unsafe { buf . as_mut_vec () } , bytes) ; Poll :: Ready (ret) }
    };
}

read_line_internal!()