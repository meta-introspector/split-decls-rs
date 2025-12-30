// Generated macro for v8_metadata_as_current (function)
macro_rules! Depcrate_file_formats_v8v8_metadata_as_current {
() => {
// Module: crate::file_formats::v8
// Provides: {"v8_metadata_as_current"}
// Dependencies: {}
fn v8_metadata_as_current (old : & OldMetadata) -> Metadata { Metadata { start_time : old . start_time , process_id : old . process_id , cmd : old . cmd . clone () , } }
};
}
