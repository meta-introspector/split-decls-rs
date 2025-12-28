macro_rules! deps {
    () => {
        Result!();
        StringTable!();
        Endian!();
        ReadRef!();
        Vernaux!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > elf :: Vernaux < Endian > { # [doc = " Parse the version name from the string table."] pub fn name < 'data , R : ReadRef < 'data > > (& self , endian : Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . vna_name . get (endian)) . read_error ("Invalid ELF vna_name") } }
    };
}

impl_445!()