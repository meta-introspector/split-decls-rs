macro_rules! deps {
    () => {
        RangeEncoder!();
        RangeEncoderBuffer!();
        Result!();
        LzmaEncoderTrait!();
        LzmaEncoder!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl LzmaEncoder { pub fn encode_for_lzma2 (& mut self , rc : & mut RangeEncoder < RangeEncoderBuffer > , mode : & mut dyn LzmaEncoderTrait ,) -> crate :: Result < bool > { if ! self . lz . is_started () && ! self . encode_init (rc) ? { return Ok (false) ; } while self . data . uncompressed_size <= LZMA2_UNCOMPRESSED_LIMIT && rc . get_pending_size () <= LZMA2_COMPRESSED_LIMIT { if ! self . encode_symbol (rc , mode) ? { return Ok (false) ; } } Ok (true) } }
    };
}

impl_186!()