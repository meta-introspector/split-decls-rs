// Generated macro for location_info (function)
macro_rules! Depcratelocation_info {
() => {
// Module: crate
// Provides: {"location_info"}
// Dependencies: {}
fn location_info (locs : & Option < Locations > , frame : & Frame , current_dir : & Path) -> LocationInfo { let (mut file , mut line , mut mod_path) = (None , None , None) ; let loc = locs . as_ref () . map (| locs | locs . get (& frame . index ())) ; if let Some (Some (loc)) = loc { let path = loc . file . strip_prefix (current_dir) . unwrap_or (& loc . file) ; file = Some (path . display () . to_string ()) ; line = Some (loc . line as u32) ; mod_path = Some (loc . module . clone ()) ; } (file , line , mod_path) }
};
}
