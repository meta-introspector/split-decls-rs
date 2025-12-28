macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! decompress_all_at_once_with {
    () => {
        deps!();
        fn decompress_all_at_once_with (inflate : & mut zlib :: Inflate , b : & [u8] , decompressed_len : usize , out : & mut Vec < u8 > ,) -> Result < () , Error > { out . resize (decompressed_len , 0) ; inflate . reset () ; inflate . once (b , out) . map_err (| err | Error :: ZlibInflate { source : err , message : "Failed to decompress entry" , }) ? ; Ok (()) }
    };
}

decompress_all_at_once_with!()