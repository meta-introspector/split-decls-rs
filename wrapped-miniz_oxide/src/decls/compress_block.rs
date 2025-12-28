macro_rules! deps {
    () => {
        LZOxide!();
        OutputBufferOxide!();
        Result!();
        HuffmanOxide!();
    };
}

macro_rules! compress_block {
    () => {
        deps!();
        fn compress_block (huff : & mut HuffmanOxide , output : & mut OutputBufferOxide , lz : & LZOxide , static_block : bool ,) -> Result < bool > { if static_block { huff . start_static_block (output) ; } else { huff . start_dynamic_block (output) ? ; } compress_lz_codes (huff , output , & lz . codes , lz . code_position) }
    };
}

compress_block!();