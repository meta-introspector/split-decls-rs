macro_rules! deps {
    () => {
        DebugAbbrev!();
        Abbreviations!();
        Result!();
        Reader!();
        DebugAbbrevOffset!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < R : Reader > DebugAbbrev < R > { # [doc = " Parse the abbreviations at the given `offset` within this"] # [doc = " `.debug_abbrev` section."] # [doc = ""] # [doc = " The `offset` should generally be retrieved from a unit header."] pub fn abbreviations (& self , debug_abbrev_offset : DebugAbbrevOffset < R :: Offset > ,) -> Result < Abbreviations > { let input = & mut self . debug_abbrev_section . clone () ; input . skip (debug_abbrev_offset . 0) ? ; Abbreviations :: parse (input) } }
    };
}

impl_334!();