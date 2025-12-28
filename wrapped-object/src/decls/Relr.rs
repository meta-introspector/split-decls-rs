macro_rules! deps {
    () => {
        Endian!();
        Pod!();
        Relr64!();
        Relr32!();
    };
}

macro_rules! Relr {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`elf::Relr32`] and [`elf::Relr64`]."] # [allow (missing_docs)] pub trait Relr : Debug + Pod + Clone { type Word : Into < u64 > ; type Endian : endian :: Endian ; # [doc = " The number of bits in the bit mask, excluding the lowest bit."] const COUNT : u8 ; # [doc = " Get the relocation entry."] # [doc = ""] # [doc = " This value is an offset if the lowest bit is clear, or a bit mask if the lowest bit is set."] fn get (& self , endian : Self :: Endian) -> Self :: Word ; # [doc = " Return the offset corresponding to the next bit in the bit mask."] # [doc = ""] # [doc = " Updates the offset and bit mask. This method should be called 31 times"] # [doc = " for Relr32 and 63 times for Relr64 to iterate over all the bits."] # [doc = ""] # [doc = " Returns `None` if the bit is not set."] fn next (offset : & mut Self :: Word , bits : & mut Self :: Word) -> Option < Self :: Word > ; }
    };
}

Relr!();