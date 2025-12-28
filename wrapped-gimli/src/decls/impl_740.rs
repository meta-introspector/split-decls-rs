macro_rules! deps {
    () => {
        AbbreviationTable!();
        LineProgram!();
        Unit!();
        DwarfUnit!();
        LineString!();
        Encoding!();
        Result!();
        Sections!();
        Writer!();
    };
}

macro_rules! impl_740 {
    () => {
        deps!();
        impl DwarfUnit { # [doc = " Create a new `DwarfUnit`."] # [doc = ""] # [doc = " Note: you should set `self.unit.line_program` after creation."] # [doc = " This cannot be done earlier because it may need to reference"] # [doc = " `self.line_strings`."] pub fn new (encoding : Encoding) -> Self { let unit = Unit :: new (encoding , LineProgram :: none ()) ; DwarfUnit { unit , line_strings : LineStringTable :: default () , strings : StringTable :: default () , } } # [doc = " Write the DWARf information to the given sections."] pub fn write < W : Writer > (& mut self , sections : & mut Sections < W >) -> Result < () > { let abbrev_offset = sections . debug_abbrev . offset () ; let mut abbrevs = AbbreviationTable :: default () ; self . unit . write (sections , abbrev_offset , & mut abbrevs , & mut self . line_strings , & mut self . strings ,) ? ; assert ! (sections . debug_info_fixups . is_empty ()) ; assert ! (sections . debug_loc_fixups . is_empty ()) ; assert ! (sections . debug_loclists_fixups . is_empty ()) ; abbrevs . write (& mut sections . debug_abbrev) ? ; self . line_strings . write (& mut sections . debug_line_str) ? ; self . strings . write (& mut sections . debug_str) ? ; Ok (()) } # [doc = " Get a reference to the data for a line string."] pub fn get_line_string < 'a > (& 'a self , string : & 'a LineString) -> & 'a [u8] { string . get (& self . strings , & self . line_strings) } }
    };
}

impl_740!()