macro_rules! deps {
    () => {
        DyldCacheSlidePointer3!();
        PtrauthKey!();
        DyldRelocation!();
        DyldRelocationAuth!();
        Result!();
        DyldCacheRelocationIteratorV3!();
        ReadRef!();
        RelocationStateV3!();
        U64!();
        Endian!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl < 'data , E , R > DyldCacheRelocationIteratorV3 < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { fn next (& mut self) -> Result < Option < DyldRelocation > > { loop { match self . state { RelocationStateV3 :: Start => { let Some (page_start) = self . page_starts . get (self . start_index) else { return Ok (None) ; } ; let page_offset = self . start_index as u64 * self . page_size ; self . start_index += 1 ; let page_start = page_start . get (self . endian) ; if page_start == macho :: DYLD_CACHE_SLIDE_V3_PAGE_ATTR_NO_REBASE { self . state = RelocationStateV3 :: Start ; } else { self . state = RelocationStateV3 :: Page ; self . offset = page_offset + u64 :: from (page_start) ; } } RelocationStateV3 :: Page => { let offset = self . offset ; let pointer = self . data . read_at :: < U64 < E > > (self . mapping_file_offset + offset) . read_error ("Invalid dyld cache slide pointer offset") ? . get (self . endian) ; let pointer = macho :: DyldCacheSlidePointer3 (pointer) ; let next = pointer . next () ; if next == 0 { self . state = RelocationStateV3 :: Start ; } else { self . offset = offset + next * 8 ; } if pointer . is_auth () { let value = pointer . runtime_offset () + self . auth_value_add ; let key = match pointer . key () { 1 => macho :: PtrauthKey :: IB , 2 => macho :: PtrauthKey :: DA , 3 => macho :: PtrauthKey :: DB , _ => macho :: PtrauthKey :: IA , } ; let auth = Some (DyldRelocationAuth { key , diversity : pointer . diversity () , addr_div : pointer . addr_div () , }) ; return Ok (Some (DyldRelocation { offset , value , auth , })) ; } else { let value = pointer . target () | pointer . high8 () << 56 ; return Ok (Some (DyldRelocation { offset , value , auth : None , })) ; } ; } } } } }
    };
}

impl_495!();