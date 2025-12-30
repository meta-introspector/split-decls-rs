// Generated macro for ConsumeBinaryHunk (struct)
macro_rules! Depcrate_blob_unified_diffConsumeBinaryHunk {
() => {
// Module: crate::blob::unified_diff
// Provides: {"ConsumeBinaryHunk"}
// Dependencies: {}
# [doc = " An adapter with [`ConsumeHunk`] implementation to call a delegate which receives each stringified hunk."] pub struct ConsumeBinaryHunk < 'a , D > { # [doc = " The newline to use to separate lines if these don't yet contain a newline."] # [doc = " It should also be used to separate the stringified header from the hunk itself."] pub newline : & 'a str , # [doc = " The delegate to receive stringified hunks."] pub delegate : D , header_buf : String , hunk_buf : Vec < u8 > , }
};
}
