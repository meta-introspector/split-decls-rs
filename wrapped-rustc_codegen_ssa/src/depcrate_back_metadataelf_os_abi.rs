// Generated macro for elf_os_abi (function)
macro_rules! Depcrate_back_metadataelf_os_abi {
() => {
// Module: crate::back::metadata
// Provides: {"elf_os_abi"}
// Dependencies: {}
pub (super) fn elf_os_abi (sess : & Session) -> u8 { match sess . target . options . os . as_ref () { "hermit" => elf :: ELFOSABI_STANDALONE , "freebsd" => elf :: ELFOSABI_FREEBSD , "solaris" => elf :: ELFOSABI_SOLARIS , _ => elf :: ELFOSABI_NONE , } }
};
}
