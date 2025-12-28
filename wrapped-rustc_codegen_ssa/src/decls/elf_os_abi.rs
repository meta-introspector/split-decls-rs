macro_rules! elf_os_abi {
    () => {
        pub (super) fn elf_os_abi (sess : & Session) -> u8 { match sess . target . options . os . as_ref () { "hermit" => elf :: ELFOSABI_STANDALONE , "freebsd" => elf :: ELFOSABI_FREEBSD , "solaris" => elf :: ELFOSABI_SOLARIS , _ => elf :: ELFOSABI_NONE , } }
    };
}

elf_os_abi!();