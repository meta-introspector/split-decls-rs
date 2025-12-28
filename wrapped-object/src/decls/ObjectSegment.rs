macro_rules! deps {
    () => {
        Result!();
        SegmentFlags!();
        Object!();
    };
}

macro_rules! ObjectSegment {
    () => {
        deps!();
        # [doc = " A loadable segment in an [`Object`]."] # [doc = ""] # [doc = " This trait is part of the unified read API."] pub trait ObjectSegment < 'data > : read :: private :: Sealed { # [doc = " Returns the virtual address of the segment."] fn address (& self) -> u64 ; # [doc = " Returns the size of the segment in memory."] fn size (& self) -> u64 ; # [doc = " Returns the alignment of the segment in memory."] fn align (& self) -> u64 ; # [doc = " Returns the offset and size of the segment in the file."] fn file_range (& self) -> (u64 , u64) ; # [doc = " Returns a reference to the file contents of the segment."] # [doc = ""] # [doc = " The length of this data may be different from the size of the"] # [doc = " segment in memory."] fn data (& self) -> Result < & 'data [u8] > ; # [doc = " Return the segment data in the given range."] # [doc = ""] # [doc = " Returns `Ok(None)` if the segment does not contain the given range."] fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > ; # [doc = " Returns the name of the segment."] fn name_bytes (& self) -> Result < Option < & [u8] > > ; # [doc = " Returns the name of the segment."] # [doc = ""] # [doc = " Returns an error if the name is not UTF-8."] fn name (& self) -> Result < Option < & str > > ; # [doc = " Return the flags of segment."] fn flags (& self) -> SegmentFlags ; }
    };
}

ObjectSegment!()