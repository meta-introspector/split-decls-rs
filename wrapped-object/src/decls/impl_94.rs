macro_rules! deps {
    () => {
        ReadRef!();
        ReadCacheOps!();
        Result!();
        ReadCacheRange!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'a , R : ReadCacheOps > ReadRef < 'a > for ReadCacheRange < 'a , R > { fn len (self) -> Result < u64 , () > { Ok (self . size) } fn read_bytes_at (self , offset : u64 , size : u64) -> Result < & 'a [u8] , () > { if size == 0 { return Ok (& []) ; } let end = offset . checked_add (size) . ok_or (()) ? ; if end > self . size { return Err (()) ; } let r_offset = self . offset . checked_add (offset) . ok_or (()) ? ; self . r . read_bytes_at (r_offset , size) } fn read_bytes_at_until (self , range : Range < u64 > , delimiter : u8) -> Result < & 'a [u8] , () > { let r_start = self . offset . checked_add (range . start) . ok_or (()) ? ; let r_end = self . offset . checked_add (range . end) . ok_or (()) ? ; let bytes = self . r . read_bytes_at_until (r_start .. r_end , delimiter) ? ; let size = bytes . len () . try_into () . map_err (| _ | ()) ? ; let end = range . start . checked_add (size) . ok_or (()) ? ; if end > self . size { return Err (()) ; } Ok (bytes) } }
    };
}

impl_94!()