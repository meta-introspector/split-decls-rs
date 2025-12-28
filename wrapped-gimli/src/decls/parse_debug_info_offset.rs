macro_rules! deps {
    () => {
        Reader!();
        Format!();
        Result!();
        DebugInfoOffset!();
    };
}

macro_rules! parse_debug_info_offset {
    () => {
        deps!();
        # [doc = " Parse the `debug_info_offset` in the arange header."] pub (crate) fn parse_debug_info_offset < R : Reader > (input : & mut R , format : Format ,) -> Result < DebugInfoOffset < R :: Offset > > { input . read_offset (format) . map (DebugInfoOffset) }
    };
}

parse_debug_info_offset!();