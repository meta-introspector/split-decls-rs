macro_rules! deps {
    () => {
        FileHeader!();
        Endian!();
        Relr!();
        RelrIterator!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > RelrIterator < 'data , Elf > { # [doc = " Create a new iterator given the `SHT_RELR` section data."] pub fn new (endian : Elf :: Endian , data : & 'data [Elf :: Relr]) -> Self { RelrIterator { offset : Elf :: Word :: default () , bits : Elf :: Word :: default () , count : 0 , iter : data . iter () , endian , } } }
    };
}

impl_368!()