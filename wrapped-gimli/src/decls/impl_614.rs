macro_rules! deps {
    () => {
        Reader!();
        UnitHeader!();
        DebugInfoUnitHeadersIter!();
        DebugInfo!();
        Result!();
        SectionId!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        impl < R : Reader > DebugInfoUnitHeadersIter < R > { # [doc = " Advance the iterator to the next unit header."] pub fn next (& mut self) -> Result < Option < UnitHeader < R > > > { if self . input . is_empty () { Ok (None) } else { let len = self . input . len () ; match parse_unit_header (& mut self . input , SectionId :: DebugInfo , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } } }
    };
}

impl_614!()