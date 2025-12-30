// Generated macro for gnome_clock_format_hc (function)
macro_rules! Depcrate_backends_linuxgnome_clock_format_hc {
() => {
// Module: crate::backends::linux
// Provides: {"gnome_clock_format_hc"}
// Dependencies: {}
# [cfg (feature = "gnome")] fn gnome_clock_format_hc () -> Option < HourCycle > { use gio :: prelude :: * ; let s = gio :: Settings :: new ("org.gnome.desktop.interface") ; match s . string ("clock-format") . as_str () { "12h" => Some (HourCycle :: H12) , "24h" => Some (HourCycle :: H23) , _ => None , } }
};
}
