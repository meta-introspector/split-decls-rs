macro_rules! deps {
    () => {
        Endianness!();
        Reader!();
        IntegerType!();
        DecodeError!();
    };
}

macro_rules! varint_decode_isize {
    () => {
        deps!();
        pub fn varint_decode_isize < R : Reader > (read : & mut R , endian : Endianness ,) -> Result < isize , DecodeError > { match varint_decode_i64 (read , endian) { Ok (val) => Ok (val as isize) , Err (DecodeError :: InvalidIntegerType { found , .. }) => { Err (DecodeError :: InvalidIntegerType { expected : IntegerType :: Isize , found : found . into_signed () , }) } Err (e) => Err (e) , } }
    };
}

varint_decode_isize!();