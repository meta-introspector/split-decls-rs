macro_rules! deps {
    () => {
        StringTable!();
        Verdaux!();
        ReadRef!();
        Result!();
        Endian!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > elf :: Verdaux < Endian > { # [doc = " Parse the version name from the string table."] pub fn name < 'data , R : ReadRef < 'data > > (& self , endian : Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . vda_name . get (endian)) . read_error ("Invalid ELF vda_name") } }
    };
}

impl_443!()