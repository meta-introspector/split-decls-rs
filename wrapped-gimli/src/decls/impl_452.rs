macro_rules! deps {
    () => {
        DebugLocListsBase!();
        ReaderOffset!();
        Encoding!();
        LocListsHeader!();
        DwarfFileType!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl < Offset > DebugLocListsBase < Offset > where Offset : ReaderOffset , { # [doc = " Returns a `DebugLocListsBase` with the default value of DW_AT_loclists_base"] # [doc = " for the given `Encoding` and `DwarfFileType`."] pub fn default_for_encoding_and_file (encoding : Encoding , file_type : DwarfFileType ,) -> DebugLocListsBase < Offset > { if encoding . version >= 5 && file_type == DwarfFileType :: Dwo { DebugLocListsBase (Offset :: from_u8 (LocListsHeader :: size_for_encoding (encoding))) } else { DebugLocListsBase (Offset :: from_u8 (0)) } } }
    };
}

impl_452!()