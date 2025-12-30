// Generated macro for build_formatted_string (function)
macro_rules! Depcrate_log_formatbuild_formatted_string {
() => {
// Module: crate::log::format
// Provides: {"build_formatted_string"}
// Dependencies: {}
fn build_formatted_string (s : & str , format : & LogFormat , default_width : usize , level : Option < Level > , log_color : Option < LogColor > ,) -> String { let s = ColoredString :: from (s) ; let styled_string_length = s . len () ; let length_without_styles = string_excluding_ansi (& s) . len () ; let length_of_ansi_sequences = styled_string_length - length_without_styles ; let s = apply_color (s , log_color , level) ; let colored_str = apply_styles (s , format . style . as_ref ()) ; let alignment = format . alignment . unwrap_or (Alignment :: Left) ; let width = format . width . unwrap_or (default_width) + length_of_ansi_sequences ; let padding = format . padding . unwrap_or (Padding :: Space) ; let mut result = String :: new () ; match (alignment , padding) { (Alignment :: Left , Padding :: Space) => write ! (& mut result , "{colored_str:<0$}" , width) , (Alignment :: Left , Padding :: Zero) => write ! (& mut result , "{colored_str:0<0$}" , width) , (Alignment :: Center , Padding :: Space) => write ! (& mut result , "{colored_str:^0$}" , width) , (Alignment :: Center , Padding :: Zero) => write ! (& mut result , "{colored_str:0^0$}" , width) , (Alignment :: Right , Padding :: Space) => write ! (& mut result , "{colored_str:>0$}" , width) , (Alignment :: Right , Padding :: Zero) => write ! (& mut result , "{colored_str:0>0$}" , width) , } . expect ("Failed to format string: \"{colored_str}\"") ; result }
};
}
