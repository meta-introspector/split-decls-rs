macro_rules! deps {
    () => {
        RawLocListEntry!();
        RawLocListIter!();
        Reader!();
        Result!();
        Encoding!();
        LocListsFormat!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl < R : Reader > RawLocListIter < R > { # [doc = " Construct a `RawLocListIter`."] fn new (input : R , encoding : Encoding , format : LocListsFormat) -> RawLocListIter < R > { RawLocListIter { input , encoding , format , } } # [doc = " Advance the iterator to the next location."] pub fn next (& mut self) -> Result < Option < RawLocListEntry < R > > > { if self . input . is_empty () { return Ok (None) ; } match RawLocListEntry :: parse (& mut self . input , self . encoding , self . format) { Ok (entry) => { if entry . is_none () { self . input . empty () ; } Ok (entry) } Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_462!();