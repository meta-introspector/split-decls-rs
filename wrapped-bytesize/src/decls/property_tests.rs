macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! property_tests {
    () => {
        deps!();
        # [cfg (test)] mod property_tests { use alloc :: string :: { String , ToString as _ } ; use super :: * ; impl quickcheck :: Arbitrary for ByteSize { fn arbitrary (g : & mut quickcheck :: Gen) -> Self { Self (u64 :: arbitrary (g)) } } quickcheck :: quickcheck ! { fn parsing_never_panics (size : String) -> bool { let _ = size . parse ::< ByteSize > () ; true } fn to_string_never_blank (size : ByteSize) -> bool { ! size . to_string () . is_empty () } fn to_string_never_large (size : ByteSize) -> bool { size . to_string () . len () < 11 } fn string_round_trip (size : ByteSize) -> bool { if size > ByteSize :: pib (1) { return true ; } size . to_string () . parse ::< ByteSize > () . unwrap () == size } } }
    };
}

property_tests!();