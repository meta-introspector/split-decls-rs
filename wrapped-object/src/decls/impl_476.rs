macro_rules! deps {
    () => {
        DyldCacheImageInfo!();
        Error!();
        Object!();
        File!();
        ReadRef!();
        Endian!();
        Result!();
        DyldCacheImage!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < 'data , 'cache , E , R > DyldCacheImage < 'data , 'cache , E , R > where E : Endian , R : ReadRef < 'data > , { # [doc = " Return the raw data structure for this image."] pub fn info (& self) -> & 'data macho :: DyldCacheImageInfo < E > { self . image_info } # [doc = " The file system path of this image."] pub fn path (& self) -> Result < & 'data str > { let path = self . image_info . path (self . cache . endian , self . cache . data) ? ; let path = core :: str :: from_utf8 (path) . map_err (| _ | Error ("Path string not valid utf-8")) ? ; Ok (path) } # [doc = " The subcache data which contains the Mach-O header for this image,"] # [doc = " together with the file offset at which this image starts."] pub fn image_data_and_offset (& self) -> Result < (R , u64) > { let address = self . image_info . address . get (self . cache . endian) ; self . cache . data_and_offset_for_address (address) . ok_or (Error ("Address not found in any mapping")) } # [doc = " Parse this image into an Object."] pub fn parse_object (& self) -> Result < File < 'data , R > > { File :: parse_dyld_cache_image (self) } }
    };
}

impl_476!()