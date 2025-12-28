macro_rules! SectionRange {
    () => {
        # [doc = " The file range and virtual address range for a section."] # [allow (missing_docs)] # [derive (Debug , Default , Clone , Copy)] pub struct SectionRange { pub virtual_address : u32 , pub virtual_size : u32 , pub file_offset : u32 , pub file_size : u32 , }
    };
}

SectionRange!();