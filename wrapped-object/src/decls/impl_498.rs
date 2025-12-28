macro_rules! deps {
    () => {
        DyldCacheRelocationIteratorV5!();
        PtrauthKey!();
        Result!();
        ReadRef!();
        DyldRelocationAuth!();
        RelocationStateV5!();
        DyldRelocation!();
        Endian!();
        U64!();
        DyldCacheSlidePointer5!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl < 'data , E , R > DyldCacheRelocationIteratorV5 < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { fn next (& mut self) -> Result < Option < DyldRelocation > > { loop { match self . state { RelocationStateV5 :: Start => { let Some (page_start) = self . page_starts . get (self . start_index) else { return Ok (None) ; } ; let page_offset = self . start_index as u64 * self . page_size ; self . start_index += 1 ; let page_start = page_start . get (self . endian) ; if page_start == macho :: DYLD_CACHE_SLIDE_V5_PAGE_ATTR_NO_REBASE { self . state = RelocationStateV5 :: Start ; } else { self . state = RelocationStateV5 :: Page ; self . offset = page_offset + u64 :: from (page_start) ; } } RelocationStateV5 :: Page => { let offset = self . offset ; let pointer = self . data . read_at :: < U64 < E > > (self . mapping_file_offset + offset) . read_error ("Invalid dyld cache slide pointer offset") ? . get (self . endian) ; let pointer = macho :: DyldCacheSlidePointer5 (pointer) ; let next = pointer . next () ; if next == 0 { self . state = RelocationStateV5 :: Start ; } else { self . offset = offset + next * 8 ; } let mut value = pointer . runtime_offset () + self . value_add ; let auth = if pointer . is_auth () { let key = if pointer . key_is_data () { macho :: PtrauthKey :: DA } else { macho :: PtrauthKey :: IA } ; Some (DyldRelocationAuth { key , diversity : pointer . diversity () , addr_div : pointer . addr_div () , }) } else { value |= pointer . high8 () << 56 ; None } ; return Ok (Some (DyldRelocation { offset , value , auth , })) ; } } } } }
    };
}

impl_498!()