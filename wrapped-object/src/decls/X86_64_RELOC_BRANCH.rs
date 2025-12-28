macro_rules! X86_64_RELOC_BRANCH {
    () => {
        # [doc = " a CALL/JMP instruction with 32-bit displacement"] pub const X86_64_RELOC_BRANCH : u8 = 2 ;
    };
}

X86_64_RELOC_BRANCH!();