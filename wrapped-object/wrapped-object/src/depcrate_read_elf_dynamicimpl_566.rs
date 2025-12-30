// Generated macro for impl_566 (impl)
macro_rules! Depcrate_read_elf_dynamicimpl_566 {
() => {
// Module: crate::read::elf::dynamic
// Provides: {"impl_566"}
// Dependencies: {}
impl < Endian : endian :: Endian > Dyn for elf :: Dyn32 < Endian > { type Word = u32 ; type Endian = Endian ; # [inline] fn d_tag (& self , endian : Self :: Endian) -> Self :: Word { self . d_tag . get (endian) } # [inline] fn d_val (& self , endian : Self :: Endian) -> Self :: Word { self . d_val . get (endian) } }
};
}
