macro_rules! deps {
    () => {
        NoHashHasher!();
        IsEnabled!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (debug_assertions)] impl < T : IsEnabled > Hasher for NoHashHasher < T > { fn write (& mut self , _ : & [u8]) { panic ! ("Invalid use of NoHashHasher") } fn write_u8 (& mut self , n : u8) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = u64 :: from (n) ; self . 1 = true } fn write_u16 (& mut self , n : u16) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = u64 :: from (n) ; self . 1 = true } fn write_u32 (& mut self , n : u32) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = u64 :: from (n) ; self . 1 = true } fn write_u64 (& mut self , n : u64) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n ; self . 1 = true } fn write_usize (& mut self , n : usize) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n as u64 ; self . 1 = true } fn write_i8 (& mut self , n : i8) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n as u64 ; self . 1 = true } fn write_i16 (& mut self , n : i16) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n as u64 ; self . 1 = true } fn write_i32 (& mut self , n : i32) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n as u64 ; self . 1 = true } fn write_i64 (& mut self , n : i64) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n as u64 ; self . 1 = true } fn write_isize (& mut self , n : isize) { assert ! (! self . 1 , "NoHashHasher: second write attempt detected.") ; self . 0 = n as u64 ; self . 1 = true } fn finish (& self) -> u64 { self . 0 } }
    };
}

impl_21!();