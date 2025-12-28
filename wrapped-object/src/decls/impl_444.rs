macro_rules! deps {
    () => {
        Result!();
        Verneed!();
        StringTable!();
        ReadRef!();
        Endian!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > elf :: Verneed < Endian > { # [doc = " Parse the file from the string table."] pub fn file < 'data , R : ReadRef < 'data > > (& self , endian : Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . vn_file . get (endian)) . read_error ("Invalid ELF vn_file") } }
    };
}

impl_444!()