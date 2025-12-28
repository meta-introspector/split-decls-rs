macro_rules! deps {
    () => {
        SectionId!();
        DebugTypesUnitHeadersIter!();
        Reader!();
        DebugTypes!();
        UnitHeader!();
        Result!();
    };
}

macro_rules! impl_661 {
    () => {
        deps!();
        impl < R : Reader > DebugTypesUnitHeadersIter < R > { # [doc = " Advance the iterator to the next type unit header."] pub fn next (& mut self) -> Result < Option < UnitHeader < R > > > { if self . input . is_empty () { Ok (None) } else { let len = self . input . len () ; match parse_unit_header (& mut self . input , SectionId :: DebugTypes , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } } }
    };
}

impl_661!();