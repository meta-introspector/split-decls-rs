// Generated macro for rel_flag_type (function)
macro_rules! Depcrate_readobj_elfrel_flag_type {
() => {
// Module: crate::readobj::elf
// Provides: {"rel_flag_type"}
// Dependencies: {}
fn rel_flag_type < Elf : FileHeader > (endian : Elf :: Endian , elf : & Elf) -> & 'static [Flag < u32 >] { match elf . e_machine (endian) { EM_68K => FLAGS_R_68K , EM_386 => FLAGS_R_386 , EM_SPARC => FLAGS_R_SPARC , EM_MIPS => FLAGS_R_MIPS , EM_PARISC => FLAGS_R_PARISC , EM_ALPHA => FLAGS_R_ALPHA , EM_PPC => FLAGS_R_PPC , EM_PPC64 => FLAGS_R_PPC64 , EM_AARCH64 => FLAGS_R_AARCH64 , EM_ARM => FLAGS_R_ARM , EM_CSKY => FLAGS_R_CKCORE , EM_IA_64 => FLAGS_R_IA64 , EM_SH => FLAGS_R_SH , EM_S390 => FLAGS_R_390 , EM_CRIS => FLAGS_R_CRIS , EM_X86_64 => FLAGS_R_X86_64 , EM_MN10300 => FLAGS_R_MN10300 , EM_M32R => FLAGS_R_M32R , EM_MICROBLAZE => FLAGS_R_MICROBLAZE , EM_ALTERA_NIOS2 => FLAGS_R_NIOS2 , EM_TILEPRO => FLAGS_R_TILEPRO , EM_TILEGX => FLAGS_R_TILEGX , EM_RISCV => FLAGS_R_RISCV , EM_BPF => FLAGS_R_BPF , EM_SBF => FLAGS_R_SBF , EM_LOONGARCH => FLAGS_R_LOONGARCH , EM_METAG => FLAGS_R_METAG , EM_NDS32 => FLAGS_R_NDS32 , _ => & [] , } }
};
}
