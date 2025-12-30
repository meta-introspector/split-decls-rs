// Generated macro for capture_handler (function)
macro_rules! Depcratecapture_handler {
() => {
// Module: crate
// Provides: {"capture_handler"}
// Dependencies: {}
# [cfg_attr (track_caller , track_caller)] # [cfg_attr (not (track_caller) , allow (unused_mut))] fn capture_handler (error : & (dyn StdError + 'static)) -> Box < dyn EyreHandler > { # [cfg (not (feature = "auto-install"))] let hook = HOOK . get () . expect ("a handler must always be installed if the `auto-install` feature is disabled") . as_ref () ; # [cfg (feature = "auto-install")] let hook = HOOK . get_or_init (| | Box :: new (DefaultHandler :: default_with)) . as_ref () ; let mut handler = hook (error) ; # [cfg (track_caller)] { handler . track_caller (std :: panic :: Location :: caller ()) } handler }
};
}
