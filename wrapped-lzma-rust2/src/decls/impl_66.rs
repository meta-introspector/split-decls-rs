macro_rules! deps {
    () => {
        Read!();
        Result!();
        LzipHeader!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl LzipHeader { fn parse < R : Read > (reader : & mut R) -> Result < Self > { let mut magic = [0u8 ; 4] ; reader . read_exact (& mut magic) ? ; if magic != LZIP_MAGIC { return Err (error_invalid_data ("invalid LZIP magic bytes")) ; } let version = reader . read_u8 () ? ; if version != LZIP_VERSION { return Err (error_invalid_data ("unsupported LZIP version")) ; } let dict_size_byte = reader . read_u8 () ? ; let dict_size = decode_dict_size (dict_size_byte) ? ; Ok (LzipHeader { version , dict_size }) } }
    };
}

impl_66!()