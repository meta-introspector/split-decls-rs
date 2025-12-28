macro_rules! deps {
    () => {
        DyldCacheMappingIterator!();
        DyldCacheMappingVersionIterator!();
        Endian!();
        ReadRef!();
        DyldFile!();
        DyldCacheMappingSlice!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < 'data , E , R > DyldFile < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { # [doc = " Return an iterator for the mappings."] fn mappings (& self , endian : E) -> DyldCacheMappingIterator < 'data , E , R > { let iter = match self . mappings { DyldCacheMappingSlice :: V1 (info) => DyldCacheMappingVersionIterator :: V1 (info . iter ()) , DyldCacheMappingSlice :: V2 (info) => DyldCacheMappingVersionIterator :: V2 (info . iter ()) , } ; DyldCacheMappingIterator { endian , data : self . data , iter , } } # [doc = " Find the file offset an address in the mappings."] fn address_to_file_offset (& self , endian : E , address : u64) -> Option < u64 > { for mapping in self . mappings (endian) { let mapping_address = mapping . address () ; if address >= mapping_address && address < mapping_address . wrapping_add (mapping . size ()) { return Some (address - mapping_address + mapping . file_offset ()) ; } } None } }
    };
}

impl_472!();