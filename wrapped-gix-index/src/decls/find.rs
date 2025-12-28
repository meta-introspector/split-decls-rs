macro_rules! deps {
    () => {
        Iter!();
        Offset!();
    };
}

macro_rules! find {
    () => {
        deps!();
        pub fn find (extensions : & [u8] , object_hash : gix_hash :: Kind) -> Option < Vec < Offset > > { extension :: Iter :: new_without_checksum (extensions , object_hash) ? . find_map (| (sig , ext_data) | (sig == SIGNATURE) . then_some (ext_data)) . and_then (decode) }
    };
}

find!()