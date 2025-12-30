// Generated macro for Config (struct)
macro_rules! Depcrate_summaryConfig {
() => {
// Module: crate::summary
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration for the summary printer."] # [doc = ""] # [doc = " This is manipulated by the SummaryBuilder and then referenced by the actual"] # [doc = " implementation. Once a printer is build, the configuration is frozen and"] # [doc = " cannot changed."] # [derive (Debug , Clone)] struct Config { kind : SummaryKind , colors : ColorSpecs , hyperlink : HyperlinkConfig , stats : bool , path : bool , exclude_zero : bool , separator_field : Arc < Vec < u8 > > , separator_path : Option < u8 > , path_terminator : Option < u8 > , }
};
}
