macro_rules! deps {
    () => {
        Result!();
        Error!();
        DyldCacheMappingSlice!();
        DyldCacheImageInfo!();
        DyldSubCacheEntryV1!();
        DyldSubCacheEntryV2!();
        Architecture!();
        DyldCacheMappingInfo!();
        DyldSubCacheSlice!();
        DyldCacheHeader!();
        ReadRef!();
        DyldCacheMappingAndSlideInfo!();
        Endian!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl < E : Endian > macho :: DyldCacheHeader < E > { # [doc = " Read the dyld cache header."] pub fn parse < 'data , R : ReadRef < 'data > > (data : R) -> Result < & 'data Self > { data . read_at :: < macho :: DyldCacheHeader < E > > (0) . read_error ("Invalid dyld cache header size or alignment") } # [doc = " Returns (arch, endian) based on the magic string."] pub fn parse_magic (& self) -> Result < (Architecture , E) > { let (arch , is_big_endian) = match & self . magic { b"dyld_v1    i386\0" => (Architecture :: I386 , false) , b"dyld_v1  x86_64\0" => (Architecture :: X86_64 , false) , b"dyld_v1 x86_64h\0" => (Architecture :: X86_64 , false) , b"dyld_v1     ppc\0" => (Architecture :: PowerPc , true) , b"dyld_v1   armv6\0" => (Architecture :: Arm , false) , b"dyld_v1   armv7\0" => (Architecture :: Arm , false) , b"dyld_v1  armv7f\0" => (Architecture :: Arm , false) , b"dyld_v1  armv7s\0" => (Architecture :: Arm , false) , b"dyld_v1  armv7k\0" => (Architecture :: Arm , false) , b"dyld_v1   arm64\0" => (Architecture :: Aarch64 , false) , b"dyld_v1  arm64e\0" => (Architecture :: Aarch64 , false) , _ => return Err (Error ("Unrecognized dyld cache magic")) , } ; let endian = E :: from_big_endian (is_big_endian) . read_error ("Unsupported dyld cache endian") ? ; Ok ((arch , endian)) } # [doc = " Return the mapping information table."] pub fn mappings < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < DyldCacheMappingSlice < 'data , E > > { let header_size = self . mapping_offset . get (endian) ; if header_size >= MIN_HEADER_SIZE_MAPPINGS_V2 { let info = data . read_slice_at :: < macho :: DyldCacheMappingAndSlideInfo < E > > (self . mapping_with_slide_offset . get (endian) . into () , self . mapping_with_slide_count . get (endian) as usize ,) . read_error ("Invalid dyld cache mapping size or alignment") ? ; Ok (DyldCacheMappingSlice :: V2 (info)) } else { let info = data . read_slice_at :: < macho :: DyldCacheMappingInfo < E > > (self . mapping_offset . get (endian) . into () , self . mapping_count . get (endian) as usize ,) . read_error ("Invalid dyld cache mapping size or alignment") ? ; Ok (DyldCacheMappingSlice :: V1 (info)) } } # [doc = " Return the information about subcaches, if present."] # [doc = ""] # [doc = " Returns `None` for dyld caches produced before dyld-940 (macOS 12)."] pub fn subcaches < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < Option < DyldSubCacheSlice < 'data , E > > > { let header_size = self . mapping_offset . get (endian) ; if header_size >= MIN_HEADER_SIZE_SUBCACHES_V2 { let subcaches = data . read_slice_at :: < macho :: DyldSubCacheEntryV2 < E > > (self . sub_cache_array_offset . get (endian) . into () , self . sub_cache_array_count . get (endian) as usize ,) . read_error ("Invalid dyld subcaches size or alignment") ? ; Ok (Some (DyldSubCacheSlice :: V2 (subcaches))) } else if header_size >= MIN_HEADER_SIZE_SUBCACHES_V1 { let subcaches = data . read_slice_at :: < macho :: DyldSubCacheEntryV1 < E > > (self . sub_cache_array_offset . get (endian) . into () , self . sub_cache_array_count . get (endian) as usize ,) . read_error ("Invalid dyld subcaches size or alignment") ? ; Ok (Some (DyldSubCacheSlice :: V1 (subcaches))) } else { Ok (None) } } # [doc = " Return the UUID for the .symbols subcache, if present."] pub fn symbols_subcache_uuid (& self , endian : E) -> Option < [u8 ; 16] > { if self . mapping_offset . get (endian) >= MIN_HEADER_SIZE_SUBCACHES_V1 { let uuid = self . symbol_file_uuid ; if uuid != [0 ; 16] { return Some (uuid) ; } } None } # [doc = " Return the image information table."] pub fn images < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < & 'data [macho :: DyldCacheImageInfo < E >] > { if self . mapping_offset . get (endian) >= MIN_HEADER_SIZE_SUBCACHES_V1 { data . read_slice_at :: < macho :: DyldCacheImageInfo < E > > (self . images_offset . get (endian) . into () , self . images_count . get (endian) as usize ,) . read_error ("Invalid dyld cache image size or alignment") } else { data . read_slice_at :: < macho :: DyldCacheImageInfo < E > > (self . images_offset_old . get (endian) . into () , self . images_count_old . get (endian) as usize ,) . read_error ("Invalid dyld cache image size or alignment") } } }
    };
}

impl_503!();