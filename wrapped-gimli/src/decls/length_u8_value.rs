macro_rules! deps {
    () => {
        Reader!();
        Result!();
    };
}

macro_rules! length_u8_value {
    () => {
        deps!();
        fn length_u8_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_u8 () . map (R :: Offset :: from_u8) ? ; input . split (len) }
    };
}

length_u8_value!()