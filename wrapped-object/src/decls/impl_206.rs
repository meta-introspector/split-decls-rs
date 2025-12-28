macro_rules! deps {
    () => {
        StringTable!();
        CoffHeader!();
        Result!();
        ImageSectionHeader!();
        SectionTable!();
        Item!();
        SectionIndex!();
        ReadRef!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'data > SectionTable < 'data > { # [doc = " Parse the section table."] # [doc = ""] # [doc = " `data` must be the entire file data."] # [doc = " `offset` must be after the optional file header."] pub fn parse < Coff : CoffHeader , R : ReadRef < 'data > > (header : & Coff , data : R , offset : u64 ,) -> Result < Self > { let sections = data . read_slice_at (offset , header . number_of_sections () as usize) . read_error ("Invalid COFF/PE section headers") ? ; Ok (SectionTable { sections }) } # [doc = " Iterate over the section headers."] # [doc = ""] # [doc = " Warning: section indices start at 1."] # [inline] pub fn iter (& self) -> slice :: Iter < 'data , pe :: ImageSectionHeader > { self . sections . iter () } # [doc = " Iterate over the section headers and their indices."] pub fn enumerate (& self) -> impl Iterator < Item = (SectionIndex , & 'data pe :: ImageSectionHeader) > { self . sections . iter () . enumerate () . map (| (i , section) | (SectionIndex (i + 1) , section)) } # [doc = " Return true if the section table is empty."] # [inline] pub fn is_empty (& self) -> bool { self . sections . is_empty () } # [doc = " The number of section headers."] # [inline] pub fn len (& self) -> usize { self . sections . len () } # [doc = " Return the section header at the given index."] # [doc = ""] # [doc = " The index is 1-based."] pub fn section (& self , index : SectionIndex) -> read :: Result < & 'data pe :: ImageSectionHeader > { self . sections . get (index . 0 . wrapping_sub (1)) . read_error ("Invalid COFF/PE section index") } # [doc = " Return the section header with the given name."] # [doc = ""] # [doc = " The returned index is 1-based."] # [doc = ""] # [doc = " Ignores sections with invalid names."] pub fn section_by_name < R : ReadRef < 'data > > (& self , strings : StringTable < 'data , R > , name : & [u8] ,) -> Option < (SectionIndex , & 'data pe :: ImageSectionHeader) > { self . enumerate () . find (| (_ , section) | section . name (strings) == Ok (name)) } # [doc = " Compute the maximum file offset used by sections."] # [doc = ""] # [doc = " This will usually match the end of file, unless the PE file has a"] # [doc = " [data overlay](https://security.stackexchange.com/questions/77336/how-is-the-file-overlay-read-by-an-exe-virus)"] pub fn max_section_file_offset (& self) -> u64 { let mut max = 0 ; for section in self . iter () { match (section . pointer_to_raw_data . get (LE) as u64) . checked_add (section . size_of_raw_data . get (LE) as u64) { None => { continue ; } Some (end_of_section) => { if end_of_section > max { max = end_of_section ; } } } } max } }
    };
}

impl_206!()