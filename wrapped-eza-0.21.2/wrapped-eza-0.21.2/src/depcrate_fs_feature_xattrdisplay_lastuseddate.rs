// Generated macro for display_lastuseddate (function)
macro_rules! Depcrate_fs_feature_xattrdisplay_lastuseddate {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"display_lastuseddate"}
// Dependencies: {}
# [cfg (target_os = "macos")] fn display_lastuseddate (attribute : & Attribute) -> Option < String > { use chrono :: { Local , SecondsFormat , TimeZone } ; attribute . value . as_ref () . filter (| value | value . len () == 16) . and_then (| value | { let sec = i64 :: from_le_bytes (value [0 .. 8] . try_into () . unwrap ()) ; let n_sec = i64 :: from_le_bytes (value [8 ..] . try_into () . unwrap ()) ; Local . timestamp_opt (sec , n_sec as u32) . map (| dt | dt . to_rfc3339_opts (SecondsFormat :: Nanos , true)) . single () }) }
};
}
