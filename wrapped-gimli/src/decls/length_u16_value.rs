macro_rules! deps {
    () => {
        Reader!();
        Result!();
    };
}

macro_rules! length_u16_value {
    () => {
        deps!();
        fn length_u16_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_u16 () . map (R :: Offset :: from_u16) ? ; input . split (len) }
    };
}

length_u16_value!();