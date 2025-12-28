macro_rules! deps {
    () => {
        DebugAbbrev!();
        Abbreviation!();
        AbbreviationTable!();
        Writer!();
        Result!();
    };
}

macro_rules! impl_711 {
    () => {
        deps!();
        impl AbbreviationTable { # [doc = " Add an abbreviation to the table and return its code."] pub fn add (& mut self , abbrev : Abbreviation) -> u64 { let (code , _) = self . abbrevs . insert_full (abbrev) ; (code + 1) as u64 } # [doc = " Write the abbreviation table to the `.debug_abbrev` section."] pub fn write < W : Writer > (& self , w : & mut DebugAbbrev < W >) -> Result < () > { for (code , abbrev) in self . abbrevs . iter () . enumerate () { w . write_uleb128 ((code + 1) as u64) ? ; abbrev . write (w) ? ; } w . write_u8 (0) } }
    };
}

impl_711!();