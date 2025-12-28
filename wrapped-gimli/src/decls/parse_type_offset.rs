macro_rules! deps {
    () => {
        Format!();
        UnitOffset!();
        Result!();
        Reader!();
    };
}

macro_rules! parse_type_offset {
    () => {
        deps!();
        # [doc = " Parse a type unit header's type offset."] fn parse_type_offset < R : Reader > (input : & mut R , format : Format) -> Result < UnitOffset < R :: Offset > > { input . read_offset (format) . map (UnitOffset) }
    };
}

parse_type_offset!()