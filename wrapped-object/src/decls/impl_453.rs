macro_rules! deps {
    () => {
        AttributesSubsubsectionIterator!();
        AttributesSubsection!();
        FileHeader!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > AttributesSubsection < 'data , Elf > { # [doc = " Return the length of the attributes subsection."] pub fn length (& self) -> u32 { self . length } # [doc = " Return the vendor name of the attributes subsection."] pub fn vendor (& self) -> & 'data [u8] { self . vendor } # [doc = " Return an iterator over the sub-subsections."] pub fn subsubsections (& self) -> AttributesSubsubsectionIterator < 'data , Elf > { AttributesSubsubsectionIterator { endian : self . endian , data : self . data , } } }
    };
}

impl_453!();