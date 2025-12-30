// Generated macro for impl_593 (impl)
macro_rules! Depcrate_read_elf_noteimpl_593 {
() => {
// Module: crate::read::elf::note
// Provides: {"impl_593"}
// Dependencies: {}
impl < Endian : endian :: Endian > NoteHeader for elf :: NoteHeader32 < Endian > { type Endian = Endian ; # [inline] fn n_namesz (& self , endian : Self :: Endian) -> u32 { self . n_namesz . get (endian) } # [inline] fn n_descsz (& self , endian : Self :: Endian) -> u32 { self . n_descsz . get (endian) } # [inline] fn n_type (& self , endian : Self :: Endian) -> u32 { self . n_type . get (endian) } }
};
}
