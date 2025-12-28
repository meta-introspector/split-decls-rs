macro_rules! deps {
    () => {
        Endian!();
        Result!();
        Bytes!();
        U32!();
        GnuProperty!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < 'data > GnuProperty < 'data > { # [doc = " Return the property type."] # [doc = ""] # [doc = " This is one of the `GNU_PROPERTY_*` constants."] pub fn pr_type (& self) -> u32 { self . pr_type } # [doc = " Return the property data."] pub fn pr_data (& self) -> & 'data [u8] { self . pr_data } # [doc = " Parse the property data as an unsigned 32-bit integer."] pub fn data_u32 < E : endian :: Endian > (& self , endian : E) -> read :: Result < u32 > { Bytes (self . pr_data) . read_at :: < U32 < E > > (0) . read_error ("Invalid ELF GNU property data") . map (| val | val . get (endian)) } }
    };
}

impl_417!()