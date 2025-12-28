macro_rules! ReadFromSlice {
    () => {
        pub (crate) trait ReadFromSlice { fn read_u16 (& self) -> (u16 , & [u8]) ; fn read_u32 (& self) -> (u32 , & [u8]) ; fn read_u64 (& self) -> (u64 , & [u8]) ; fn read_u128 (& self) -> (u128 , & [u8]) ; fn read_u128x2 (& self) -> ([u128 ; 2] , & [u8]) ; fn read_u128x4 (& self) -> ([u128 ; 4] , & [u8]) ; fn read_last_u16 (& self) -> u16 ; fn read_last_u32 (& self) -> u32 ; fn read_last_u64 (& self) -> u64 ; fn read_last_u128 (& self) -> u128 ; fn read_last_u128x2 (& self) -> [u128 ; 2] ; fn read_last_u128x4 (& self) -> [u128 ; 4] ; }
    };
}

ReadFromSlice!()