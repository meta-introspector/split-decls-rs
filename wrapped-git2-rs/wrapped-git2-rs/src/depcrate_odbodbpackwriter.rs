// Generated macro for OdbPackwriter (struct)
macro_rules! Depcrate_odbOdbPackwriter {
() => {
// Module: crate::odb
// Provides: {"OdbPackwriter"}
// Dependencies: {}
# [doc = " A stream to write a packfile to the ODB"] pub struct OdbPackwriter < 'repo > { raw : * mut raw :: git_odb_writepack , progress : raw :: git_indexer_progress , progress_payload_ptr : * mut OdbPackwriterCb < 'repo > , }
};
}
