macro_rules! deps {
    () => {
        Reader!();
        Result!();
    };
}

macro_rules! length_uleb128_value {
    () => {
        deps!();
        fn length_uleb128_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_uleb128 () . and_then (R :: Offset :: from_u64) ? ; input . split (len) }
    };
}

length_uleb128_value!();