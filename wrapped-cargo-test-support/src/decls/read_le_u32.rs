macro_rules! read_le_u32 {
    () => {
        fn read_le_u32 < R > (mut reader : R) -> io :: Result < u32 > where R : Read , { let mut buf = [0 ; 4] ; reader . read_exact (& mut buf) ? ; Ok (u32 :: from_le_bytes (buf)) }
    };
}

read_le_u32!()