macro_rules! read_small {
    () => {
        # [doc = " Given a small (less than 8 byte slice) returns the same data stored in two u32s."] # [doc = " (order of and non-duplication of bytes is NOT guaranteed)"] # [inline (always)] pub (crate) fn read_small (data : & [u8]) -> [u64 ; 2] { debug_assert ! (data . len () <= 8) ; if data . len () >= 2 { if data . len () >= 4 { [data . read_u32 () . 0 as u64 , data . read_last_u32 () as u64] } else { [data . read_u16 () . 0 as u64 , data [data . len () - 1] as u64] } } else { if data . len () > 0 { [data [0] as u64 , data [0] as u64] } else { [0 , 0] } } }
    };
}

read_small!()