// Generated macro for Designators (struct)
macro_rules! Depcrate_fmt_friendly_printerDesignators {
() => {
// Module: crate::fmt::friendly::printer
// Provides: {"Designators"}
// Dependencies: {}
# [doc = " A type that represents the designator choice."] # [doc = ""] # [doc = " Basically, whether we want verbose, short or compact designators. This in"] # [doc = " turn permits lookups based on `Unit`, which makes writing generic code for"] # [doc = " writing designators a bit nicer and still fast."] # [derive (Debug)] struct Designators { singular : & 'static [& 'static str] , plural : & 'static [& 'static str] , }
};
}
