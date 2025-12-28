macro_rules! deps {
    () => {
        Segment!();
        U64!();
        Endian!();
        U32!();
    };
}

macro_rules! ProgramHeader64 {
    () => {
        deps!();
        # [doc = " Program segment header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ProgramHeader64 < E : Endian > { # [doc = " Segment type. One of the `PT_*` constants."] pub p_type : U32 < E > , # [doc = " Segment flags. A combination of the `PF_*` constants."] pub p_flags : U32 < E > , # [doc = " Segment file offset."] pub p_offset : U64 < E > , # [doc = " Segment virtual address."] pub p_vaddr : U64 < E > , # [doc = " Segment physical address."] pub p_paddr : U64 < E > , # [doc = " Segment size in the file."] pub p_filesz : U64 < E > , # [doc = " Segment size in memory."] pub p_memsz : U64 < E > , # [doc = " Segment alignment."] pub p_align : U64 < E > , }
    };
}

ProgramHeader64!();