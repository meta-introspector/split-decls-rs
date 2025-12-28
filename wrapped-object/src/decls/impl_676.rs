macro_rules! deps {
    () => {
        ReadRef!();
        SectionTable!();
        ImageSectionHeader!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl < 'data > SectionTable < 'data > { # [doc = " Return the file offset of the given virtual address, and the size up"] # [doc = " to the end of the section containing it."] # [doc = ""] # [doc = " Returns `None` if no section contains the address."] pub fn pe_file_range_at (& self , va : u32) -> Option < (u32 , u32) > { self . iter () . find_map (| section | section . pe_file_range_at (va)) } # [doc = " Return the data starting at the given virtual address, up to the end of the"] # [doc = " section containing it."] # [doc = ""] # [doc = " Ignores sections with invalid data."] # [doc = ""] # [doc = " Returns `None` if no section contains the address."] pub fn pe_data_at < R : ReadRef < 'data > > (& self , data : R , va : u32) -> Option < & 'data [u8] > { self . iter () . find_map (| section | section . pe_data_at (data , va)) } # [doc = " Return the data of the section that contains the given virtual address in a PE file."] # [doc = ""] # [doc = " Also returns the virtual address of that section."] # [doc = ""] # [doc = " Ignores sections with invalid data."] pub fn pe_data_containing < R : ReadRef < 'data > > (& self , data : R , va : u32 ,) -> Option < (& 'data [u8] , u32) > { self . iter () . find_map (| section | section . pe_data_containing (data , va)) } # [doc = " Return the section that contains a given virtual address."] pub fn section_containing (& self , va : u32) -> Option < & 'data ImageSectionHeader > { self . iter () . find (| section | section . contains_rva (va)) } }
    };
}

impl_676!()