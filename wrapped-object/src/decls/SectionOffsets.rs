macro_rules! SectionOffsets {
    () => {
        # [derive (Default , Clone , Copy)] struct SectionOffsets { address : u64 , data_offset : usize , reloc_offset : usize , }
    };
}

SectionOffsets!();