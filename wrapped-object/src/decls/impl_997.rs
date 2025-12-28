macro_rules! deps {
    () => {
        U16!();
        WritableBuffer!();
        MachO!();
        U32!();
        MachHeader32!();
        SegmentCommand!();
        SectionHeader!();
        BigEndian!();
        MachO32!();
        Nlist!();
        Nlist32!();
        Section32!();
        MachHeader!();
        SegmentCommand32!();
        Endian!();
    };
}

macro_rules! impl_997 {
    () => {
        deps!();
        impl < E : Endian > MachO for MachO32 < E > { fn mach_header_size (& self) -> usize { mem :: size_of :: < macho :: MachHeader32 < E > > () } fn segment_command_size (& self) -> usize { mem :: size_of :: < macho :: SegmentCommand32 < E > > () } fn section_header_size (& self) -> usize { mem :: size_of :: < macho :: Section32 < E > > () } fn nlist_size (& self) -> usize { mem :: size_of :: < macho :: Nlist32 < E > > () } fn write_mach_header (& self , buffer : & mut dyn WritableBuffer , header : MachHeader) { let endian = self . endian ; let magic = if endian . is_big_endian () { macho :: MH_MAGIC } else { macho :: MH_CIGAM } ; let header = macho :: MachHeader32 { magic : U32 :: new (BigEndian , magic) , cputype : U32 :: new (endian , header . cputype) , cpusubtype : U32 :: new (endian , header . cpusubtype) , filetype : U32 :: new (endian , header . filetype) , ncmds : U32 :: new (endian , header . ncmds) , sizeofcmds : U32 :: new (endian , header . sizeofcmds) , flags : U32 :: new (endian , header . flags) , } ; buffer . write (& header) ; } fn write_segment_command (& self , buffer : & mut dyn WritableBuffer , segment : SegmentCommand) { let endian = self . endian ; let segment = macho :: SegmentCommand32 { cmd : U32 :: new (endian , macho :: LC_SEGMENT) , cmdsize : U32 :: new (endian , segment . cmdsize) , segname : segment . segname , vmaddr : U32 :: new (endian , segment . vmaddr as u32) , vmsize : U32 :: new (endian , segment . vmsize as u32) , fileoff : U32 :: new (endian , segment . fileoff as u32) , filesize : U32 :: new (endian , segment . filesize as u32) , maxprot : U32 :: new (endian , segment . maxprot) , initprot : U32 :: new (endian , segment . initprot) , nsects : U32 :: new (endian , segment . nsects) , flags : U32 :: new (endian , segment . flags) , } ; buffer . write (& segment) ; } fn write_section (& self , buffer : & mut dyn WritableBuffer , section : SectionHeader) { let endian = self . endian ; let section = macho :: Section32 { sectname : section . sectname , segname : section . segname , addr : U32 :: new (endian , section . addr as u32) , size : U32 :: new (endian , section . size as u32) , offset : U32 :: new (endian , section . offset) , align : U32 :: new (endian , section . align) , reloff : U32 :: new (endian , section . reloff) , nreloc : U32 :: new (endian , section . nreloc) , flags : U32 :: new (endian , section . flags) , reserved1 : U32 :: default () , reserved2 : U32 :: default () , } ; buffer . write (& section) ; } fn write_nlist (& self , buffer : & mut dyn WritableBuffer , nlist : Nlist) { let endian = self . endian ; let nlist = macho :: Nlist32 { n_strx : U32 :: new (endian , nlist . n_strx) , n_type : nlist . n_type , n_sect : nlist . n_sect , n_desc : U16 :: new (endian , nlist . n_desc) , n_value : U32 :: new (endian , nlist . n_value as u32) , } ; buffer . write (& nlist) ; } }
    };
}

impl_997!();