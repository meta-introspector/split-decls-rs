macro_rules! deps {
    () => {
        Reader!();
        Result!();
    };
}

macro_rules! length_u32_value {
    () => {
        deps!();
        fn length_u32_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_u32 () . map (R :: Offset :: from_u32) ? ; input . split (len) }
    };
}

length_u32_value!()