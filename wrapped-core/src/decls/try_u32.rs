macro_rules! try_u32 {
    () => {
        fn try_u32 (bytes : & mut core :: str :: Bytes , delimiter : bool) -> Result < u32 > { next (bytes , 8 , delimiter) . ok_or_else (invalid_guid) }
    };
}

try_u32!();