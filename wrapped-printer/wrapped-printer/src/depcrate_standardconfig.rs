// Generated macro for Config (struct)
macro_rules! Depcrate_standardConfig {
() => {
// Module: crate::standard
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration for the standard printer."] # [doc = ""] # [doc = " This is manipulated by the StandardBuilder and then referenced by the"] # [doc = " actual implementation. Once a printer is build, the configuration is frozen"] # [doc = " and cannot changed."] # [derive (Debug , Clone)] struct Config { colors : ColorSpecs , hyperlink : HyperlinkConfig , stats : bool , heading : bool , path : bool , only_matching : bool , per_match : bool , per_match_one_line : bool , replacement : Arc < Option < Vec < u8 > > > , max_columns : Option < u64 > , max_columns_preview : bool , column : bool , byte_offset : bool , trim_ascii : bool , separator_search : Arc < Option < Vec < u8 > > > , separator_context : Arc < Option < Vec < u8 > > > , separator_field_match : Arc < Vec < u8 > > , separator_field_context : Arc < Vec < u8 > > , separator_path : Option < u8 > , path_terminator : Option < u8 > , }
};
}
