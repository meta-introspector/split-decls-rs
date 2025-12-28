macro_rules! deps {
    () => {
        Result!();
        Symbol32!();
        StringTable!();
        ReadRef!();
        Symbol!();
    };
}

macro_rules! impl_828 {
    () => {
        deps!();
        impl Symbol for xcoff :: Symbol32 { type Word = u32 ; fn n_value (& self) -> Self :: Word { self . n_value . get (BE) } fn n_scnum (& self) -> i16 { self . n_scnum . get (BE) } fn n_type (& self) -> u16 { self . n_type . get (BE) } fn n_sclass (& self) -> u8 { self . n_sclass } fn n_numaux (& self) -> u8 { self . n_numaux } fn name_offset (& self) -> Option < u32 > { if self . n_name [0] == 0 { let offset = u32 :: from_be_bytes (self . n_name [4 .. 8] . try_into () . unwrap ()) ; Some (offset) } else { None } } # [doc = " Parse the symbol name for XCOFF32."] fn name < 'data , R : ReadRef < 'data > > (& 'data self , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { if let Some (offset) = self . name_offset () { strings . get (offset) . read_error ("Invalid XCOFF symbol name offset") } else { Ok (match memchr :: memchr (b'\0' , & self . n_name) { Some (end) => & self . n_name [.. end] , None => & self . n_name , }) } } }
    };
}

impl_828!();