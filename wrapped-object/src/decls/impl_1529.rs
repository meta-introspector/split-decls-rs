macro_rules! deps {
    () => {
        Endian!();
        Rel32!();
        U32!();
    };
}

macro_rules! impl_1529 {
    () => {
        deps!();
        impl < E : Endian > Rel32 < E > { # [doc = " Get the `r_sym` component of the `r_info` field."] # [inline] pub fn r_sym (& self , endian : E) -> u32 { self . r_info . get (endian) >> 8 } # [doc = " Get the `r_type` component of the `r_info` field."] # [inline] pub fn r_type (& self , endian : E) -> u32 { self . r_info . get (endian) & 0xff } # [doc = " Calculate the `r_info` field given the `r_sym` and `r_type` components."] pub fn r_info (endian : E , r_sym : u32 , r_type : u8) -> U32 < E > { U32 :: new (endian , (r_sym << 8) | u32 :: from (r_type)) } # [doc = " Set the `r_info` field given the `r_sym` and `r_type` components."] pub fn set_r_info (& mut self , endian : E , r_sym : u32 , r_type : u8) { self . r_info = Self :: r_info (endian , r_sym , r_type) } }
    };
}

impl_1529!()