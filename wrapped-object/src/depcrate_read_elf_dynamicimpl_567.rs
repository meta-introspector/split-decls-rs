// Generated macro for impl_567 (impl)
macro_rules! Depcrate_read_elf_dynamicimpl_567 {
() => {
// Module: crate::read::elf::dynamic
// Provides: {"impl_567"}
// Dependencies: {}
impl < Endian : endian :: Endian > Dyn for elf :: Dyn64 < Endian > { type Word = u64 ; type Endian = Endian ; # [inline] fn d_tag (& self , endian : Self :: Endian) -> Self :: Word { self . d_tag . get (endian) } # [inline] fn d_val (& self , endian : Self :: Endian) -> Self :: Word { self . d_val . get (endian) } }
};
}
