macro_rules! deps {
    () => {
        SectionIndex!();
        Result!();
        ReadRef!();
        SectionTable!();
        RelocationSections!();
        Endian!();
        FileHeader!();
        Error!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl RelocationSections { # [doc = " Create a new mapping using the section table."] # [doc = ""] # [doc = " Skips relocation sections that do not use the given symbol table section."] pub fn parse < 'data , Elf : FileHeader , R : ReadRef < 'data > > (endian : Elf :: Endian , sections : & SectionTable < 'data , Elf , R > , symbol_section : SectionIndex ,) -> read :: Result < Self > { let mut relocations = vec ! [0 ; sections . len ()] ; for (index , section) in sections . iter () . enumerate () . rev () { let sh_type = section . sh_type (endian) ; if sh_type == elf :: SHT_REL || sh_type == elf :: SHT_RELA || sh_type == elf :: SHT_CREL { let sh_link = section . link (endian) ; if sh_link != symbol_section { continue ; } let sh_info = section . info_link (endian) ; if sh_info == SectionIndex (0) { continue ; } if sh_info . 0 >= relocations . len () { return Err (Error ("Invalid ELF sh_info for relocation section")) ; } let sh_info_type = sections . section (sh_info) ? . sh_type (endian) ; if sh_info_type == elf :: SHT_REL || sh_info_type == elf :: SHT_RELA || sh_info_type == elf :: SHT_CREL { return Err (Error ("Unsupported ELF sh_info for relocation section")) ; } let next = relocations [sh_info . 0] ; relocations [sh_info . 0] = index ; relocations [index] = next ; } } Ok (Self { relocations }) } # [doc = " Given a section index, return the section index of the associated relocation section."] # [doc = ""] # [doc = " This may also be called with a relocation section index, and it will return the"] # [doc = " next associated relocation section."] pub fn get (& self , index : SectionIndex) -> Option < SectionIndex > { self . relocations . get (index . 0) . cloned () . filter (| x | * x != 0) . map (SectionIndex) } }
    };
}

impl_346!();