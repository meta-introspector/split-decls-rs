// Generated macro for BakedExporterCloseMetadata (struct)
macro_rules! Depcrate_exportBakedExporterCloseMetadata {
() => {
// Module: crate::export
// Provides: {"BakedExporterCloseMetadata"}
// Dependencies: {}
# [doc = " Metadata of a bake export"] pub struct BakedExporterCloseMetadata { # [doc = " Per-marker size heuristics"] pub statistics : BTreeMap < DataMarkerInfo , Statistics > , # [doc = " List of crates required to compile the output"] pub required_crates : BTreeSet < & 'static str > , }
};
}
