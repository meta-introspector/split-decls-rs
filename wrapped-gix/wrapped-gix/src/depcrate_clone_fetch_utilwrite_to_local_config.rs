// Generated macro for write_to_local_config (function)
macro_rules! Depcrate_clone_fetch_utilwrite_to_local_config {
() => {
// Module: crate::clone::fetch::util
// Provides: {"write_to_local_config"}
// Dependencies: {}
fn write_to_local_config (config : & gix_config :: File < 'static > , mode : WriteMode) -> std :: io :: Result < () > { assert_eq ! (config . meta () . source , gix_config :: Source :: Local , "made for appending to local configuration file") ; let mut local_config = std :: fs :: OpenOptions :: new () . create (false) . write (matches ! (mode , WriteMode :: Overwrite)) . append (matches ! (mode , WriteMode :: Append)) . open (config . meta () . path . as_deref () . expect ("local config with path set")) ? ; local_config . write_all (config . detect_newline_style ()) ? ; config . write_to_filter (& mut local_config , | s | s . meta () . source == gix_config :: Source :: Local) }
};
}
