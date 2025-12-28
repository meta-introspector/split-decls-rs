macro_rules! deps {
    () => {
        SectionTable!();
        ImageDataDirectory!();
        Result!();
        Error!();
        ReadRef!();
    };
}

macro_rules! impl_683 {
    () => {
        deps!();
        impl pe :: ImageDataDirectory { # [doc = " Return the virtual address range of this directory entry."] pub fn address_range (& self) -> (u32 , u32) { (self . virtual_address . get (LE) , self . size . get (LE)) } # [doc = " Return the file offset and size of this directory entry."] # [doc = ""] # [doc = " This function has some limitations:"] # [doc = " - It requires that the data is contained in a single section."] # [doc = " - It uses the size field of the directory entry, which is"] # [doc = "   not desirable for all data directories."] # [doc = " - It uses the `virtual_address` of the directory entry as an address,"] # [doc = "   which is not valid for `IMAGE_DIRECTORY_ENTRY_SECURITY`."] pub fn file_range (& self , sections : & SectionTable < '_ >) -> Result < (u32 , u32) > { let (offset , section_size) = sections . pe_file_range_at (self . virtual_address . get (LE)) . read_error ("Invalid data dir virtual address") ? ; let size = self . size . get (LE) ; if size > section_size { return Err (Error ("Invalid data dir size")) ; } Ok ((offset , size)) } # [doc = " Get the data referenced by this directory entry."] # [doc = ""] # [doc = " This function has some limitations:"] # [doc = " - It requires that the data is contained in a single section."] # [doc = " - It uses the size field of the directory entry, which is"] # [doc = "   not desirable for all data directories."] # [doc = " - It uses the `virtual_address` of the directory entry as an address,"] # [doc = "   which is not valid for `IMAGE_DIRECTORY_ENTRY_SECURITY`."] pub fn data < 'data , R : ReadRef < 'data > > (& self , data : R , sections : & SectionTable < 'data > ,) -> Result < & 'data [u8] > { sections . pe_data_at (data , self . virtual_address . get (LE)) . read_error ("Invalid data dir virtual address") ? . get (.. self . size . get (LE) as usize) . read_error ("Invalid data dir size") } }
    };
}

impl_683!();