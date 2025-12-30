// Generated macro for impl_375 (impl)
macro_rules! Depcrate_read_endian_readerimpl_375 {
() => {
// Module: crate::read::endian_reader
// Provides: {"impl_375"}
// Dependencies: {}
impl < Endian , T > EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { # [doc = " Construct a new `EndianReader` with the given bytes."] # [inline] pub fn new (bytes : T , endian : Endian) -> EndianReader < Endian , T > { EndianReader { range : SubRange :: new (bytes) , endian , } } # [doc = " Return a reference to the raw bytes underlying this reader."] # [inline] pub fn bytes (& self) -> & [u8] { self . range . bytes () } }
};
}
