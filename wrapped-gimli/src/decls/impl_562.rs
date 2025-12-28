macro_rules! deps {
    () => {
        DwarfFileType!();
        RngListsHeader!();
        DebugRngListsBase!();
        ReaderOffset!();
        Encoding!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl < Offset > DebugRngListsBase < Offset > where Offset : ReaderOffset , { # [doc = " Returns a `DebugRngListsBase` with the default value of DW_AT_rnglists_base"] # [doc = " for the given `Encoding` and `DwarfFileType`."] pub fn default_for_encoding_and_file (encoding : Encoding , file_type : DwarfFileType ,) -> DebugRngListsBase < Offset > { if encoding . version >= 5 && file_type == DwarfFileType :: Dwo { DebugRngListsBase (Offset :: from_u8 (RngListsHeader :: size_for_encoding (encoding))) } else { DebugRngListsBase (Offset :: from_u8 (0)) } } }
    };
}

impl_562!();