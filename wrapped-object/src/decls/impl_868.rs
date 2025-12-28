macro_rules! deps {
    () => {
        ObjectSegment!();
        FileHeader!();
        XcoffSegment!();
        Result!();
        SegmentFlags!();
        ReadRef!();
    };
}

macro_rules! impl_868 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > ObjectSegment < 'data > for XcoffSegment < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { fn address (& self) -> u64 { unreachable ! () ; } fn size (& self) -> u64 { unreachable ! () ; } fn align (& self) -> u64 { unreachable ! () ; } fn file_range (& self) -> (u64 , u64) { unreachable ! () ; } fn data (& self) -> Result < & 'data [u8] > { unreachable ! () ; } fn data_range (& self , _address : u64 , _size : u64) -> Result < Option < & 'data [u8] > > { unreachable ! () ; } fn name_bytes (& self) -> Result < Option < & [u8] > > { unreachable ! () ; } fn name (& self) -> Result < Option < & str > > { unreachable ! () ; } fn flags (& self) -> SegmentFlags { unreachable ! () ; } }
    };
}

impl_868!();