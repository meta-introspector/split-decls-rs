// Generated macro for to_header (function)
macro_rules! Depcrate_data_input_entryto_header {
() => {
// Module: crate::data::input::entry
// Provides: {"to_header"}
// Dependencies: {}
fn to_header (kind : gix_object :: Kind) -> Header { use gix_object :: Kind :: * ; match kind { Tree => Header :: Tree , Blob => Header :: Blob , Commit => Header :: Commit , Tag => Header :: Tag , } }
};
}
