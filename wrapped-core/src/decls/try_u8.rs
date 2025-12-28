macro_rules! try_u8 {
    () => {
        fn try_u8 (bytes : & mut core :: str :: Bytes , delimiter : bool) -> Result < u8 > { next (bytes , 2 , delimiter) . map (| value | value as u8) . ok_or_else (invalid_guid) }
    };
}

try_u8!()