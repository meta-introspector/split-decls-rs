// Generated macro for impl_570 (impl)
macro_rules! Depcrate_read_loclistsimpl_570 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_570"}
// Dependencies: {}
impl < Offset > DebugLocListsBase < Offset > where Offset : ReaderOffset , { # [doc = " Returns a `DebugLocListsBase` with the default value of DW_AT_loclists_base"] # [doc = " for the given `Encoding` and `DwarfFileType`."] pub fn default_for_encoding_and_file (encoding : Encoding , file_type : DwarfFileType ,) -> DebugLocListsBase < Offset > { if encoding . version >= 5 && file_type == DwarfFileType :: Dwo { DebugLocListsBase (Offset :: from_u8 (LocListsHeader :: size_for_encoding (encoding))) } else { DebugLocListsBase (Offset :: from_u8 (0)) } } }
};
}
