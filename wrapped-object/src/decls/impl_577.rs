macro_rules! deps {
    () => {
        Endian!();
        Segment!();
        Section!();
        SegmentCommand32!();
        Section32!();
        LoadCommandData!();
        Result!();
    };
}

macro_rules! impl_577 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > Segment for macho :: SegmentCommand32 < Endian > { type Word = u32 ; type Endian = Endian ; type Section = macho :: Section32 < Self :: Endian > ; fn from_command (command : LoadCommandData < '_ , Self :: Endian >) -> Result < Option < (& Self , & [u8]) > > { command . segment_32 () } fn cmd (& self , endian : Self :: Endian) -> u32 { self . cmd . get (endian) } fn cmdsize (& self , endian : Self :: Endian) -> u32 { self . cmdsize . get (endian) } fn segname (& self) -> & [u8 ; 16] { & self . segname } fn vmaddr (& self , endian : Self :: Endian) -> Self :: Word { self . vmaddr . get (endian) } fn vmsize (& self , endian : Self :: Endian) -> Self :: Word { self . vmsize . get (endian) } fn fileoff (& self , endian : Self :: Endian) -> Self :: Word { self . fileoff . get (endian) } fn filesize (& self , endian : Self :: Endian) -> Self :: Word { self . filesize . get (endian) } fn maxprot (& self , endian : Self :: Endian) -> u32 { self . maxprot . get (endian) } fn initprot (& self , endian : Self :: Endian) -> u32 { self . initprot . get (endian) } fn nsects (& self , endian : Self :: Endian) -> u32 { self . nsects . get (endian) } fn flags (& self , endian : Self :: Endian) -> u32 { self . flags . get (endian) } }
    };
}

impl_577!()