macro_rules! deps {
    () => {
        AHasher!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [doc = " Provides [Hasher] methods to hash all of the primitive types."] # [doc = ""] # [doc = " [Hasher]: core::hash::Hasher"] impl Hasher for AHasher { # [inline] fn write_u8 (& mut self , i : u8) { self . update (i as u64) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . update (i as u64) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . update (i as u64) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . update (i) ; } # [inline] fn write_u128 (& mut self , i : u128) { self . large_update (i) ; } # [inline] # [cfg (any (target_pointer_width = "64" , target_pointer_width = "32" , target_pointer_width = "16"))] fn write_usize (& mut self , i : usize) { self . write_u64 (i as u64) ; } # [inline] # [cfg (target_pointer_width = "128")] fn write_usize (& mut self , i : usize) { self . write_u128 (i as u128) ; } # [inline] # [allow (clippy :: collapsible_if)] fn write (& mut self , input : & [u8]) { let mut data = input ; let length = data . len () as u64 ; self . buffer = self . buffer . wrapping_add (length) . wrapping_mul (MULTIPLE) ; if data . len () > 8 { if data . len () > 16 { let tail = data . read_last_u128 () ; self . large_update (tail) ; while data . len () > 16 { let (block , rest) = data . read_u128 () ; self . large_update (block) ; data = rest ; } } else { self . large_update ([data . read_u64 () . 0 , data . read_last_u64 ()] . convert ()) ; } } else { let value = read_small (data) ; self . large_update (value . convert ()) ; } } # [inline] fn finish (& self) -> u64 { let rot = (self . buffer & 63) as u32 ; folded_multiply (self . buffer , self . pad) . rotate_left (rot) } }
    };
}

impl_19!();