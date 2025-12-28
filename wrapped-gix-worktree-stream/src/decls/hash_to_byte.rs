macro_rules! hash_to_byte {
    () => {
        fn hash_to_byte (h : gix_hash :: Kind) -> u8 { match h { gix_hash :: Kind :: Sha1 => 0 , } }
    };
}

hash_to_byte!()