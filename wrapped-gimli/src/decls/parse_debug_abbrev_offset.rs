macro_rules! deps {
    () => {
        DebugAbbrevOffset!();
        Result!();
        Format!();
        Reader!();
    };
}

macro_rules! parse_debug_abbrev_offset {
    () => {
        deps!();
        # [doc = " Parse the `debug_abbrev_offset` in the compilation unit header."] fn parse_debug_abbrev_offset < R : Reader > (input : & mut R , format : Format ,) -> Result < DebugAbbrevOffset < R :: Offset > > { input . read_offset (format) . map (DebugAbbrevOffset) }
    };
}

parse_debug_abbrev_offset!();