macro_rules! deps {
    () => {
        U16!();
        U32!();
        Endian!();
        DyldCacheSlideInfo3!();
        DyldCacheSlideInfo5!();
        DyldCacheSlideInfo!();
        DyldCacheMappingAndSlideInfo!();
        ReadRef!();
        Result!();
        DyldCacheSlideInfo2!();
        Error!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < E : Endian > macho :: DyldCacheMappingAndSlideInfo < E > { # [doc = " Return the (optional) array of slide information structs"] pub fn slide < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < DyldCacheSlideInfo < 'data , E > > { if self . slide_info_file_size . get (endian) == 0 { return Ok (DyldCacheSlideInfo :: None) ; } let slide_info_file_offset = self . slide_info_file_offset . get (endian) ; let version = data . read_at :: < U32 < E > > (slide_info_file_offset) . read_error ("Invalid slide info file offset size or alignment") ? . get (endian) ; match version { 2 => { let slide = data . read_at :: < macho :: DyldCacheSlideInfo2 < E > > (slide_info_file_offset) . read_error ("Invalid dyld cache slide info offset or alignment") ? ; let page_starts_offset = slide_info_file_offset . checked_add (slide . page_starts_offset . get (endian) as u64) . read_error ("Invalid dyld cache page starts offset") ? ; let page_starts = data . read_slice_at :: < U16 < E > > (page_starts_offset , slide . page_starts_count . get (endian) as usize ,) . read_error ("Invalid dyld cache page starts size or alignment") ? ; let page_extras_offset = slide_info_file_offset . checked_add (slide . page_extras_offset . get (endian) as u64) . read_error ("Invalid dyld cache page extras offset") ? ; let page_extras = data . read_slice_at :: < U16 < E > > (page_extras_offset , slide . page_extras_count . get (endian) as usize ,) . read_error ("Invalid dyld cache page extras size or alignment") ? ; Ok (DyldCacheSlideInfo :: V2 { slide , page_starts , page_extras , }) } 3 => { let slide = data . read_at :: < macho :: DyldCacheSlideInfo3 < E > > (slide_info_file_offset) . read_error ("Invalid dyld cache slide info offset or alignment") ? ; let page_starts_offset = slide_info_file_offset . checked_add (mem :: size_of :: < macho :: DyldCacheSlideInfo3 < E > > () as u64) . read_error ("Invalid dyld cache page starts offset") ? ; let page_starts = data . read_slice_at :: < U16 < E > > (page_starts_offset , slide . page_starts_count . get (endian) as usize ,) . read_error ("Invalid dyld cache page starts size or alignment") ? ; Ok (DyldCacheSlideInfo :: V3 { slide , page_starts }) } 5 => { let slide = data . read_at :: < macho :: DyldCacheSlideInfo5 < E > > (slide_info_file_offset) . read_error ("Invalid dyld cache slide info offset or alignment") ? ; let page_starts_offset = slide_info_file_offset . checked_add (mem :: size_of :: < macho :: DyldCacheSlideInfo5 < E > > () as u64) . read_error ("Invalid dyld cache page starts offset") ? ; let page_starts = data . read_slice_at :: < U16 < E > > (page_starts_offset , slide . page_starts_count . get (endian) as usize ,) . read_error ("Invalid dyld cache page starts size or alignment") ? ; Ok (DyldCacheSlideInfo :: V5 { slide , page_starts }) } _ => Err (Error ("Unsupported dyld cache slide info version")) , } } }
    };
}

impl_505!();