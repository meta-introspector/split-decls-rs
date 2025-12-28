macro_rules! deps {
    () => {
        Result!();
        AttrsIter!();
        Attribute!();
        Reader!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < 'abbrev , 'entry , 'unit , R : Reader > AttrsIter < 'abbrev , 'entry , 'unit , R > { # [doc = " Advance the iterator and return the next attribute."] # [doc = ""] # [doc = " Returns `None` when iteration is finished. If an error"] # [doc = " occurs while parsing the next attribute, then this error"] # [doc = " is returned, and all subsequent calls return `None`."] # [inline (always)] pub fn next (& mut self) -> Result < Option < Attribute < R > > > { if self . attributes . is_empty () { if let Some (end) = self . entry . attrs_len . get () { debug_assert_eq ! (end , self . input . offset_from (& self . entry . attrs_slice)) ; } else { self . entry . attrs_len . set (Some (self . input . offset_from (& self . entry . attrs_slice))) ; } return Ok (None) ; } let spec = self . attributes [0] ; let rest_spec = & self . attributes [1 ..] ; match parse_attribute (& mut self . input , self . entry . unit . encoding () , spec) { Ok (attr) => { self . attributes = rest_spec ; Ok (Some (attr)) } Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_640!();