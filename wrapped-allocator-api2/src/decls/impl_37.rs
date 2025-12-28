macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : ? Sized + Hasher , A : Allocator > Hasher for Box < T , A > { # [inline (always)] fn finish (& self) -> u64 { (* * self) . finish () } # [inline (always)] fn write (& mut self , bytes : & [u8]) { (* * self) . write (bytes) } # [inline (always)] fn write_u8 (& mut self , i : u8) { (* * self) . write_u8 (i) } # [inline (always)] fn write_u16 (& mut self , i : u16) { (* * self) . write_u16 (i) } # [inline (always)] fn write_u32 (& mut self , i : u32) { (* * self) . write_u32 (i) } # [inline (always)] fn write_u64 (& mut self , i : u64) { (* * self) . write_u64 (i) } # [inline (always)] fn write_u128 (& mut self , i : u128) { (* * self) . write_u128 (i) } # [inline (always)] fn write_usize (& mut self , i : usize) { (* * self) . write_usize (i) } # [inline (always)] fn write_i8 (& mut self , i : i8) { (* * self) . write_i8 (i) } # [inline (always)] fn write_i16 (& mut self , i : i16) { (* * self) . write_i16 (i) } # [inline (always)] fn write_i32 (& mut self , i : i32) { (* * self) . write_i32 (i) } # [inline (always)] fn write_i64 (& mut self , i : i64) { (* * self) . write_i64 (i) } # [inline (always)] fn write_i128 (& mut self , i : i128) { (* * self) . write_i128 (i) } # [inline (always)] fn write_isize (& mut self , i : isize) { (* * self) . write_isize (i) } }
    };
}

impl_37!()