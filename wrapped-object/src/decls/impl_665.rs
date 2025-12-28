macro_rules! deps {
    () => {
        ReadRef!();
        ObjectSegment!();
        SegmentFlags!();
        PeSegment!();
        ImageNtHeaders!();
        Result!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > ObjectSegment < 'data > for PeSegment < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [inline] fn address (& self) -> u64 { u64 :: from (self . section . virtual_address . get (LE)) . wrapping_add (self . file . common . image_base) } # [inline] fn size (& self) -> u64 { u64 :: from (self . section . virtual_size . get (LE)) } # [inline] fn align (& self) -> u64 { self . file . section_alignment () } # [inline] fn file_range (& self) -> (u64 , u64) { let (offset , size) = self . section . pe_file_range () ; (u64 :: from (offset) , u64 :: from (size)) } fn data (& self) -> Result < & 'data [u8] > { self . section . pe_data (self . file . data) } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . data () ? , self . address () , address , size ,)) } # [inline] fn name_bytes (& self) -> Result < Option < & [u8] > > { self . section . name (self . file . common . symbols . strings ()) . map (Some) } # [inline] fn name (& self) -> Result < Option < & str > > { let name = self . section . name (self . file . common . symbols . strings ()) ? ; Ok (Some (str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 PE section name") ? ,)) } # [inline] fn flags (& self) -> SegmentFlags { let characteristics = self . section . characteristics . get (LE) ; SegmentFlags :: Coff { characteristics } } }
    };
}

impl_665!()