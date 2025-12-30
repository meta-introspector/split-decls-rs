// Generated macro for impl_195 (impl)
macro_rules! Depcrate_lengthimpl_195 {
() => {
// Module: crate::length
// Provides: {"impl_195"}
// Dependencies: {}
impl Encode for Length { fn encoded_len (& self) -> Result < Length > { match self . inner { 0 ..= 0x7F => Ok (Length :: new (1)) , 0x80 ..= 0xFF => Ok (Length :: new (2)) , 0x100 ..= 0xFFFF => Ok (Length :: new (3)) , 0x10000 ..= 0xFFFFFF => Ok (Length :: new (4)) , 0x1000000 ..= 0xFFFFFFFF => Ok (Length :: new (5)) , } } fn encode (& self , writer : & mut impl Writer) -> Result < () > { match self . initial_octet () { Some (tag_byte) => { writer . write_byte (tag_byte) ? ; match self . inner . to_be_bytes () { [0 , 0 , 0 , byte] => writer . write_byte (byte) , [0 , 0 , bytes @ ..] => writer . write (& bytes) , [0 , bytes @ ..] => writer . write (& bytes) , bytes => writer . write (& bytes) , } } # [allow (clippy :: cast_possible_truncation)] None => writer . write_byte (self . inner as u8) , } } }
};
}
