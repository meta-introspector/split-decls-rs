macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Format { # [doc = " Return the serialized size of an initial length field for the format."] # [inline] pub fn initial_length_size (self) -> u8 { match self { Format :: Dwarf32 => 4 , Format :: Dwarf64 => 12 , } } # [doc = " Return the natural word size for the format"] # [inline] pub fn word_size (self) -> u8 { match self { Format :: Dwarf32 => 4 , Format :: Dwarf64 => 8 , } } }
    };
}

impl_3!();