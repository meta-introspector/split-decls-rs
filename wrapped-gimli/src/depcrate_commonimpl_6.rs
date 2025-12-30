// Generated macro for impl_6 (impl)
macro_rules! Depcrate_commonimpl_6 {
() => {
// Module: crate::common
// Provides: {"impl_6"}
// Dependencies: {}
impl Format { # [doc = " Return the serialized size of an initial length field for the format."] # [inline] pub fn initial_length_size (self) -> u8 { match self { Format :: Dwarf32 => 4 , Format :: Dwarf64 => 12 , } } # [doc = " Return the natural word size for the format"] # [inline] pub fn word_size (self) -> u8 { match self { Format :: Dwarf32 => 4 , Format :: Dwarf64 => 8 , } } }
};
}
