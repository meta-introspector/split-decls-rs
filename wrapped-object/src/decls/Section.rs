macro_rules! deps {
    () => {
        SectionId!();
        ByteString!();
        SectionData!();
        Sections!();
    };
}

macro_rules! Section {
    () => {
        deps!();
        # [doc = " A section in [`Sections`]."] # [doc = ""] # [doc = " This corresponds to [`elf::SectionHeader32`] or [`elf::SectionHeader64`]."] # [derive (Debug)] pub struct Section < 'data > { id : SectionId , # [doc = " Ignore this section when writing the ELF file."] pub delete : bool , # [doc = " The name of the section."] # [doc = ""] # [doc = " This is automatically added to the section header string table,"] # [doc = " and the resulting string table offset is used to set the `sh_name`"] # [doc = " field in the ELF section header."] pub name : ByteString < 'data > , # [doc = " The `sh_type` field in the ELF section header."] # [doc = ""] # [doc = " One of the `SHT_*` constants."] pub sh_type : u32 , # [doc = " The `sh_flags` field in the ELF section header."] # [doc = ""] # [doc = " A combination of the `SHF_*` constants."] pub sh_flags : u64 , # [doc = " The `sh_addr` field in the ELF section header."] pub sh_addr : u64 , # [doc = " The `sh_offset` field in the ELF section header."] # [doc = ""] # [doc = " This is the file offset of the data in the section."] # [doc = " Writing will fail if the data cannot be placed at this offset."] # [doc = ""] # [doc = " This is only used for sections that have `SHF_ALLOC` set."] # [doc = " For other sections, the section data is written at the next available"] # [doc = " offset."] pub sh_offset : u64 , # [doc = " The `sh_size` field in the ELF section header."] # [doc = ""] # [doc = " This size is not used when writing. The size of the `data` field is"] # [doc = " used instead."] pub sh_size : u64 , # [doc = " The ID of the section linked to by the `sh_link` field in the ELF section header."] pub sh_link_section : Option < SectionId > , # [doc = " The `sh_info` field in the ELF section header."] # [doc = ""] # [doc = " Only used if `sh_info_section` is `None`."] pub sh_info : u32 , # [doc = " The ID of the section linked to by the `sh_info` field in the ELF section header."] pub sh_info_section : Option < SectionId > , # [doc = " The `sh_addralign` field in the ELF section header."] pub sh_addralign : u64 , # [doc = " The `sh_entsize` field in the ELF section header."] pub sh_entsize : u64 , # [doc = " The section data."] pub data : SectionData < 'data > , }
    };
}

Section!();