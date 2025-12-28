macro_rules! deps {
    () => {
        SectionFlags!();
        CompressedData!();
        SectionIndex!();
        RelocationMap!();
        Section!();
        Result!();
        Object!();
        Item!();
        RelocationIterator!();
        Relocation!();
        CompressedFileRange!();
        SectionKind!();
    };
}

macro_rules! ObjectSection {
    () => {
        deps!();
        # [doc = " A section in an [`Object`]."] # [doc = ""] # [doc = " This trait is part of the unified read API."] pub trait ObjectSection < 'data > : read :: private :: Sealed { # [doc = " An iterator for the relocations for a section."] # [doc = ""] # [doc = " The first field in the item tuple is the section offset"] # [doc = " that the relocation applies to."] type RelocationIterator : Iterator < Item = (u64 , Relocation) > ; # [doc = " Returns the section index."] fn index (& self) -> SectionIndex ; # [doc = " Returns the address of the section."] fn address (& self) -> u64 ; # [doc = " Returns the size of the section in memory."] fn size (& self) -> u64 ; # [doc = " Returns the alignment of the section in memory."] fn align (& self) -> u64 ; # [doc = " Returns offset and size of on-disk segment (if any)."] fn file_range (& self) -> Option < (u64 , u64) > ; # [doc = " Returns the raw contents of the section."] # [doc = ""] # [doc = " The length of this data may be different from the size of the"] # [doc = " section in memory."] # [doc = ""] # [doc = " This does not do any decompression."] fn data (& self) -> Result < & 'data [u8] > ; # [doc = " Return the raw contents of the section data in the given range."] # [doc = ""] # [doc = " This does not do any decompression."] # [doc = ""] # [doc = " Returns `Ok(None)` if the section does not contain the given range."] fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > ; # [doc = " Returns the potentially compressed file range of the section,"] # [doc = " along with information about the compression."] fn compressed_file_range (& self) -> Result < CompressedFileRange > ; # [doc = " Returns the potentially compressed contents of the section,"] # [doc = " along with information about the compression."] fn compressed_data (& self) -> Result < CompressedData < 'data > > ; # [doc = " Returns the uncompressed contents of the section."] # [doc = ""] # [doc = " The length of this data may be different from the size of the"] # [doc = " section in memory."] # [doc = ""] # [doc = " If no compression is detected, then returns the data unchanged."] # [doc = " Returns `Err` if decompression fails."] fn uncompressed_data (& self) -> Result < Cow < 'data , [u8] > > { self . compressed_data () ? . decompress () } # [doc = " Returns the name of the section."] fn name_bytes (& self) -> Result < & 'data [u8] > ; # [doc = " Returns the name of the section."] # [doc = ""] # [doc = " Returns an error if the name is not UTF-8."] fn name (& self) -> Result < & 'data str > ; # [doc = " Returns the name of the segment for this section."] fn segment_name_bytes (& self) -> Result < Option < & [u8] > > ; # [doc = " Returns the name of the segment for this section."] # [doc = ""] # [doc = " Returns an error if the name is not UTF-8."] fn segment_name (& self) -> Result < Option < & str > > ; # [doc = " Return the kind of this section."] fn kind (& self) -> SectionKind ; # [doc = " Get the relocations for this section."] fn relocations (& self) -> Self :: RelocationIterator ; # [doc = " Construct a relocation map for this section."] fn relocation_map (& self) -> Result < RelocationMap > ; # [doc = " Section flags that are specific to each file format."] fn flags (& self) -> SectionFlags ; }
    };
}

ObjectSection!()