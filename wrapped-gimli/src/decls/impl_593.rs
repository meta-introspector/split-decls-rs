macro_rules! deps {
    () => {
        DebugStrOffsetsBase!();
        ReaderOffset!();
        DwarfFileType!();
        Encoding!();
    };
}

macro_rules! impl_593 {
    () => {
        deps!();
        impl < Offset > DebugStrOffsetsBase < Offset > where Offset : ReaderOffset , { # [doc = " Returns a `DebugStrOffsetsBase` with the default value of DW_AT_str_offsets_base"] # [doc = " for the given `Encoding` and `DwarfFileType`."] pub fn default_for_encoding_and_file (encoding : Encoding , file_type : DwarfFileType ,) -> DebugStrOffsetsBase < Offset > { if encoding . version >= 5 && file_type == DwarfFileType :: Dwo { DebugStrOffsetsBase (Offset :: from_u8 (encoding . format . initial_length_size () + 2 + 2 ,)) } else { DebugStrOffsetsBase (Offset :: from_u8 (0)) } } }
    };
}

impl_593!();