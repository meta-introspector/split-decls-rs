macro_rules! bits_to_bytes {
    () => {
        # [cfg (test)] # [doc = " FIPS-203, Algorithm 3."] # [doc = " Little-endian order."] fn bits_to_bytes (bits : & [u8] , bytes : & mut [u8]) { debug_assert_eq ! (bits . len () / 8 , bytes . len ()) ; debug_assert ! (bits . iter () . all (| x | * x == 0 || * x == 1)) ; debug_assert ! (bytes . iter () . all (| x | * x == 0)) ; for (i , bit) in bits . iter () . enumerate () { let byte_idx = i >> 3 ; bytes [byte_idx] += bit * (1 << (i & 7)) ; } }
    };
}

bits_to_bytes!()