macro_rules! deps {
    () => {
        U64!();
        Endian!();
        Rela64!();
    };
}

macro_rules! impl_1537 {
    () => {
        deps!();
        impl < E : Endian > Rela64 < E > { pub (crate) fn get_r_info (& self , endian : E , is_mips64el : bool) -> u64 { let mut t = self . r_info . get (endian) ; if is_mips64el { t = (t << 32) | ((t >> 8) & 0xff000000) | ((t >> 24) & 0x00ff0000) | ((t >> 40) & 0x0000ff00) | ((t >> 56) & 0x000000ff) ; } t } # [doc = " Get the `r_sym` component of the `r_info` field."] # [inline] pub fn r_sym (& self , endian : E , is_mips64el : bool) -> u32 { (self . get_r_info (endian , is_mips64el) >> 32) as u32 } # [doc = " Get the `r_type` component of the `r_info` field."] # [inline] pub fn r_type (& self , endian : E , is_mips64el : bool) -> u32 { (self . get_r_info (endian , is_mips64el) & 0xffff_ffff) as u32 } # [doc = " Calculate the `r_info` field given the `r_sym` and `r_type` components."] pub fn r_info (endian : E , is_mips64el : bool , r_sym : u32 , r_type : u32) -> U64 < E > { let mut t = (u64 :: from (r_sym) << 32) | u64 :: from (r_type) ; if is_mips64el { t = (t >> 32) | ((t & 0xff000000) << 8) | ((t & 0x00ff0000) << 24) | ((t & 0x0000ff00) << 40) | ((t & 0x000000ff) << 56) ; } U64 :: new (endian , t) } # [doc = " Set the `r_info` field given the `r_sym` and `r_type` components."] pub fn set_r_info (& mut self , endian : E , is_mips64el : bool , r_sym : u32 , r_type : u32) { self . r_info = Self :: r_info (endian , is_mips64el , r_sym , r_type) ; } }
    };
}

impl_1537!()