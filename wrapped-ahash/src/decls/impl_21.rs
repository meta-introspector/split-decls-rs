macro_rules! deps {
    () => {
        AHasherU64!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [doc = " A specialized hasher for only primitives under 64 bits."] # [cfg (specialize)] impl Hasher for AHasherU64 { # [inline] fn finish (& self) -> u64 { folded_multiply (self . buffer , self . pad) } # [inline] fn write (& mut self , _bytes : & [u8]) { unreachable ! ("Specialized hasher was called with a different type of object") } # [inline] fn write_u8 (& mut self , i : u8) { self . write_u64 (i as u64) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . write_u64 (i as u64) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . write_u64 (i as u64) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . buffer = folded_multiply (i ^ self . buffer , MULTIPLE) ; } # [inline] fn write_u128 (& mut self , _i : u128) { unreachable ! ("Specialized hasher was called with a different type of object") } # [inline] fn write_usize (& mut self , _i : usize) { unreachable ! ("Specialized hasher was called with a different type of object") } }
    };
}

impl_21!();