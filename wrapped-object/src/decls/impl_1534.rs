macro_rules! deps {
    () => {
        Endian!();
        U64!();
        Rel64!();
    };
}

macro_rules! impl_1534 {
    () => {
        deps!();
        impl < E : Endian > Rel64 < E > { # [doc = " Get the `r_sym` component of the `r_info` field."] # [inline] pub fn r_sym (& self , endian : E) -> u32 { (self . r_info . get (endian) >> 32) as u32 } # [doc = " Get the `r_type` component of the `r_info` field."] # [inline] pub fn r_type (& self , endian : E) -> u32 { (self . r_info . get (endian) & 0xffff_ffff) as u32 } # [doc = " Calculate the `r_info` field given the `r_sym` and `r_type` components."] pub fn r_info (endian : E , r_sym : u32 , r_type : u32) -> U64 < E > { U64 :: new (endian , (u64 :: from (r_sym) << 32) | u64 :: from (r_type)) } # [doc = " Set the `r_info` field given the `r_sym` and `r_type` components."] pub fn set_r_info (& mut self , endian : E , r_sym : u32 , r_type : u32) { self . r_info = Self :: r_info (endian , r_sym , r_type) } }
    };
}

impl_1534!()