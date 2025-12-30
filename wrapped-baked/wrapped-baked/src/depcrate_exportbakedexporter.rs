// Generated macro for BakedExporter (struct)
macro_rules! Depcrate_exportBakedExporter {
() => {
// Module: crate::export
// Provides: {"BakedExporter"}
// Dependencies: {}
# [expect (clippy :: type_complexity)] # [doc = " See the module-level documentation for details."] pub struct BakedExporter { mod_directory : PathBuf , pretty : bool , use_separate_crates : bool , use_internal_fallback : bool , data : Mutex < HashMap < DataMarkerInfo , HashMap < DataPayload < ExportMarker > , BTreeSet < DataIdentifierCow < 'static > > > , > , > , # [doc = " file names, required crates, and statistics to be consumed by `close`."] impl_data : Mutex < BTreeMap < DataMarkerInfo , (SyncTokenStream , BTreeSet < & 'static str > , Statistics) > > , }
};
}
