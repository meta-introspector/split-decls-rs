macro_rules! deps {
    () => {
        Result!();
        SectionTable!();
        FileHeader!();
        SectionHeader!();
        ReadRef!();
        SectionIndex!();
    };
}

macro_rules! impl_800 {
    () => {
        deps!();
        impl < 'data , Xcoff > SectionTable < 'data , Xcoff > where Xcoff : FileHeader , { # [doc = " Parse the section table."] # [doc = ""] # [doc = " `data` must be the entire file data."] # [doc = " `offset` must be after the optional file header."] pub fn parse < R : ReadRef < 'data > > (header : & Xcoff , data : R , offset : & mut u64) -> Result < Self > { let section_num = header . f_nscns () ; if section_num == 0 { return Ok (SectionTable :: default ()) ; } let sections = data . read_slice (offset , section_num as usize) . read_error ("Invalid XCOFF section headers") ? ; Ok (SectionTable { sections }) } # [doc = " Iterate over the section headers."] # [inline] pub fn iter (& self) -> slice :: Iter < 'data , Xcoff :: SectionHeader > { self . sections . iter () } # [doc = " Return true if the section table is empty."] # [inline] pub fn is_empty (& self) -> bool { self . sections . is_empty () } # [doc = " The number of section headers."] # [inline] pub fn len (& self) -> usize { self . sections . len () } # [doc = " Return the section header at the given index."] # [doc = ""] # [doc = " The index is 1-based."] pub fn section (& self , index : SectionIndex) -> read :: Result < & 'data Xcoff :: SectionHeader > { self . sections . get (index . 0 . wrapping_sub (1)) . read_error ("Invalid XCOFF section index") } }
    };
}

impl_800!()