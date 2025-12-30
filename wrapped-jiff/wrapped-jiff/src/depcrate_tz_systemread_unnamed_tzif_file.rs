// Generated macro for read_unnamed_tzif_file (function)
macro_rules! Depcrate_tz_systemread_unnamed_tzif_file {
() => {
// Module: crate::tz::system
// Provides: {"read_unnamed_tzif_file"}
// Dependencies: {}
# [doc = " Returns the given file path as TZif data without a time zone name."] # [doc = ""] # [doc = " Normally we require TZif time zones to have a name associated with it."] # [doc = " But because there are likely platforms that hardlink /etc/localtime and"] # [doc = " perhaps have no other way to get a time zone name, we choose to support"] # [doc = " that use case. Although I cannot actually name such a platform..."] fn read_unnamed_tzif_file (path : & str) -> Result < TimeZone , Error > { let data = std :: fs :: read (path) . map_err (Error :: io) . with_context (| | err ! ("failed to read {path:?} as TZif file")) ? ; let tz = TimeZone :: tzif_system (& data) . with_context (| | err ! ("found invalid TZif data at {path:?}")) ? ; Ok (tz) }
};
}
