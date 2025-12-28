macro_rules! deps {
    () => {
        StringTable!();
        ReadRef!();
        Result!();
        Symbol64!();
        Symbol!();
    };
}

macro_rules! impl_827 {
    () => {
        deps!();
        impl Symbol for xcoff :: Symbol64 { type Word = u64 ; fn n_value (& self) -> Self :: Word { self . n_value . get (BE) } fn n_scnum (& self) -> i16 { self . n_scnum . get (BE) } fn n_type (& self) -> u16 { self . n_type . get (BE) } fn n_sclass (& self) -> u8 { self . n_sclass } fn n_numaux (& self) -> u8 { self . n_numaux } fn name_offset (& self) -> Option < u32 > { Some (self . n_offset . get (BE)) } # [doc = " Parse the symbol name for XCOFF64."] fn name < 'data , R : ReadRef < 'data > > (& 'data self , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . n_offset . get (BE)) . read_error ("Invalid XCOFF symbol name offset") } }
    };
}

impl_827!();