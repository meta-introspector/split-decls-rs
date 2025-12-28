macro_rules! try_u16 {
    () => {
        fn try_u16 (bytes : & mut core :: str :: Bytes , delimiter : bool) -> Result < u16 > { next (bytes , 4 , delimiter) . map (| value | value as u16) . ok_or_else (invalid_guid) }
    };
}

try_u16!()