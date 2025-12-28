macro_rules! deps {
    () => {
        DebugInfoOffset!();
        Reader!();
        UnitOffset!();
        PubNamesEntry!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl < R : Reader > PubNamesEntry < R > { # [doc = " Returns the name this entry refers to."] pub fn name (& self) -> & R { & self . name } # [doc = " Returns the offset into the .debug_info section for the header of the compilation unit"] # [doc = " which contains this name."] pub fn unit_header_offset (& self) -> DebugInfoOffset < R :: Offset > { self . unit_header_offset } # [doc = " Returns the offset into the compilation unit for the debugging information entry which"] # [doc = " has this name."] pub fn die_offset (& self) -> UnitOffset < R :: Offset > { self . die_offset } }
    };
}

impl_528!()