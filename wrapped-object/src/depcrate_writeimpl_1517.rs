// Generated macro for impl_1517 (impl)
macro_rules! Depcrate_writeimpl_1517 {
() => {
// Module: crate::write
// Provides: {"impl_1517"}
// Dependencies: {}
impl Mangling { # [doc = " Return the default symboling mangling for the given format and architecture."] pub fn default (format : BinaryFormat , architecture : Architecture) -> Self { match (format , architecture) { (BinaryFormat :: Coff , Architecture :: I386) => Mangling :: CoffI386 , (BinaryFormat :: Coff , _) => Mangling :: Coff , (BinaryFormat :: Elf , _) => Mangling :: Elf , (BinaryFormat :: MachO , _) => Mangling :: MachO , (BinaryFormat :: Xcoff , _) => Mangling :: Xcoff , _ => Mangling :: None , } } # [doc = " Return the prefix to use for global symbols."] pub fn global_prefix (self) -> Option < u8 > { match self { Mangling :: None | Mangling :: Elf | Mangling :: Coff | Mangling :: Xcoff => None , Mangling :: CoffI386 | Mangling :: MachO => Some (b'_') , } } }
};
}
