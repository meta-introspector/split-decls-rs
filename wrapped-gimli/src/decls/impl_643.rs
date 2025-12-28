macro_rules! deps {
    () => {
        Attribute!();
        Abbreviation!();
        Result!();
        Reader!();
        EntriesRaw!();
        UnitOffset!();
        Error!();
        AttributeSpecification!();
    };
}

macro_rules! impl_643 {
    () => {
        deps!();
        impl < 'abbrev , 'unit , R : Reader > EntriesRaw < 'abbrev , 'unit , R > { # [doc = " Return true if there is no more input."] # [inline] pub fn is_empty (& self) -> bool { self . input . is_empty () } # [doc = " Return the unit offset at which the reader will read next."] # [doc = ""] # [doc = " If you want the offset of the next entry, then this must be called prior to reading"] # [doc = " the next entry."] pub fn next_offset (& self) -> UnitOffset < R :: Offset > { UnitOffset (self . unit . header_size () + self . input . offset_from (& self . unit . entries_buf)) } # [doc = " Return the depth of the next entry."] # [doc = ""] # [doc = " This depth is updated when `read_abbreviation` is called, and is updated"] # [doc = " based on null entries and the `has_children` field in the abbreviation."] # [inline] pub fn next_depth (& self) -> isize { self . depth } # [doc = " Read an abbreviation code and lookup the corresponding `Abbreviation`."] # [doc = ""] # [doc = " Returns `Ok(None)` for null entries."] # [inline] pub fn read_abbreviation (& mut self) -> Result < Option < & 'abbrev Abbreviation > > { let code = self . input . read_uleb128 () ? ; if code == 0 { self . depth -= 1 ; return Ok (None) ; } ; let abbrev = self . abbreviations . get (code) . ok_or (Error :: UnknownAbbreviation (code)) ? ; if abbrev . has_children () { self . depth += 1 ; } Ok (Some (abbrev)) } # [doc = " Read an attribute."] # [inline] pub fn read_attribute (& mut self , spec : AttributeSpecification) -> Result < Attribute < R > > { parse_attribute (& mut self . input , self . unit . encoding () , spec) } # [doc = " Skip all the attributes of an abbreviation."] # [inline] pub fn skip_attributes (& mut self , specs : & [AttributeSpecification]) -> Result < () > { skip_attributes (& mut self . input , self . unit . encoding () , specs) } }
    };
}

impl_643!();