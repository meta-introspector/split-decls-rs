macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Blake2b!();
    };
}

macro_rules! initial_hash {
    () => {
        deps!();
        # [doc = " H0 as defined in the specification."] fn initial_hash (hash_length : u32 , memory_kib : u32 , passes : u32 , p : & [u8] , s : & [u8] , k : & [u8] , x : & [u8] ,) -> Result < [u8 ; 72] , UnknownCryptoError > { let mut h0 = [0u8 ; 72] ; let mut hasher = Blake2b :: new (BLAKE2B_OUTSIZE) ? ; h0 [0 .. 4] . copy_from_slice (& LANES . to_le_bytes ()) ; h0 [4 .. 8] . copy_from_slice (& hash_length . to_le_bytes ()) ; h0 [8 .. 12] . copy_from_slice (& memory_kib . to_le_bytes ()) ; h0 [12 .. 16] . copy_from_slice (& passes . to_le_bytes ()) ; h0 [16 .. 20] . copy_from_slice (& ARGON2_VERSION . to_le_bytes ()) ; h0 [20 .. 24] . copy_from_slice (& ARGON2_VARIANT . to_le_bytes ()) ; h0 [24 .. 28] . copy_from_slice (& (p . len () as u32) . to_le_bytes ()) ; hasher . update (& h0 [.. 28]) ? ; hasher . update (p) ? ; hasher . update (& (s . len () as u32) . to_le_bytes ()) ? ; hasher . update (s) ? ; hasher . update (& (k . len () as u32) . to_le_bytes ()) ? ; hasher . update (k) ? ; hasher . update (& (x . len () as u32) . to_le_bytes ()) ? ; hasher . update (x) ? ; h0 [0 .. BLAKE2B_OUTSIZE] . copy_from_slice (hasher . finalize () ? . as_ref ()) ; Ok (h0) }
    };
}

initial_hash!()