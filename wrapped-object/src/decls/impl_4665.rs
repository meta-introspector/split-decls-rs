macro_rules! deps {
    () => {
        ScatteredRelocationInfo!();
        Endian!();
        Relocation!();
        RelocationInfo!();
    };
}

macro_rules! impl_4665 {
    () => {
        deps!();
        impl < E : Endian > Relocation < E > { # [doc = " Determine whether this is a scattered relocation."] # [inline] pub fn r_scattered (self , endian : E , cputype : u32) -> bool { if cputype == CPU_TYPE_X86_64 { false } else { self . r_word0 . get (endian) & R_SCATTERED != 0 } } # [doc = " Return the fields of a plain relocation."] pub fn info (self , endian : E) -> RelocationInfo { let r_address = self . r_word0 . get (endian) ; let r_word1 = self . r_word1 . get (endian) ; if endian . is_little_endian () { RelocationInfo { r_address , r_symbolnum : r_word1 & 0x00ff_ffff , r_pcrel : ((r_word1 >> 24) & 0x1) != 0 , r_length : ((r_word1 >> 25) & 0x3) as u8 , r_extern : ((r_word1 >> 27) & 0x1) != 0 , r_type : (r_word1 >> 28) as u8 , } } else { RelocationInfo { r_address , r_symbolnum : r_word1 >> 8 , r_pcrel : ((r_word1 >> 7) & 0x1) != 0 , r_length : ((r_word1 >> 5) & 0x3) as u8 , r_extern : ((r_word1 >> 4) & 0x1) != 0 , r_type : (r_word1 & 0xf) as u8 , } } } # [doc = " Return the fields of a scattered relocation."] pub fn scattered_info (self , endian : E) -> ScatteredRelocationInfo { let r_word0 = self . r_word0 . get (endian) ; let r_value = self . r_word1 . get (endian) ; ScatteredRelocationInfo { r_address : r_word0 & 0x00ff_ffff , r_type : ((r_word0 >> 24) & 0xf) as u8 , r_length : ((r_word0 >> 28) & 0x3) as u8 , r_pcrel : ((r_word0 >> 30) & 0x1) != 0 , r_value , } } }
    };
}

impl_4665!();