// Generated macro for available_color_count (function)
macro_rules! Depcrate_styleavailable_color_count {
() => {
// Module: crate::style
// Provides: {"available_color_count"}
// Dependencies: {}
# [doc = " Returns available color count."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " This does not always provide a good result."] pub fn available_color_count () -> u16 { # [cfg (windows)] { if crate :: ansi_support :: supports_ansi () { return u16 :: MAX ; } } const DEFAULT : u16 = 8 ; env :: var ("COLORTERM") . or_else (| _ | env :: var ("TERM")) . map_or (DEFAULT , | x | match x { _ if x . contains ("24bit") || x . contains ("truecolor") => u16 :: MAX , _ if x . contains ("256") => 256 , _ => DEFAULT , }) }
};
}
