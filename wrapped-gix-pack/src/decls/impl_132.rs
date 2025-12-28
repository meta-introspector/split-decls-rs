macro_rules! deps {
    () => {
        Kind!();
        Entry!();
        Error!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl input :: Entry { # [doc = " Create a new input entry from a given data `obj` set to be placed at the given `pack_offset`."] # [doc = ""] # [doc = " This method is useful when arbitrary base entries are created"] pub fn from_data_obj (obj : & gix_object :: Data < '_ > , pack_offset : u64) -> Result < Self , input :: Error > { let header = to_header (obj . kind) ; let compressed = compress_data (obj) ? ; let compressed_size = compressed . len () as u64 ; let mut entry = input :: Entry { header , header_size : header . size (obj . data . len () as u64) as u16 , pack_offset , compressed : Some (compressed) , compressed_size , crc32 : None , decompressed_size : obj . data . len () as u64 , trailer : None , } ; entry . crc32 = Some (entry . compute_crc32 ()) ; Ok (entry) } # [doc = " The amount of bytes this entry may consume in a pack data file"] pub fn bytes_in_pack (& self) -> u64 { u64 :: from (self . header_size) + self . compressed_size } # [doc = " Update our CRC value by recalculating it from our header and compressed data."] pub fn compute_crc32 (& self) -> u32 { let mut header_buf = [0u8 ; 12 + gix_hash :: Kind :: longest () . len_in_bytes ()] ; let header_len = self . header . write_to (self . decompressed_size , & mut header_buf . as_mut ()) . expect ("write to memory will not fail") ; let state = gix_features :: hash :: crc32_update (0 , & header_buf [.. header_len]) ; gix_features :: hash :: crc32_update (state , self . compressed . as_ref () . expect ("we always set it")) } }
    };
}

impl_132!();