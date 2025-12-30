// Generated macro for ConsumeBinaryHunkDelegate (trait)
macro_rules! Depcrate_blob_unified_diffConsumeBinaryHunkDelegate {
() => {
// Module: crate::blob::unified_diff
// Provides: {"ConsumeBinaryHunkDelegate"}
// Dependencies: {}
# [doc = " A trait for use in conjunction with [`ConsumeBinaryHunk`]."] pub trait ConsumeBinaryHunkDelegate { # [doc = " Consume a single `hunk` in unified diff format, along with its `header_str` that already has a trailing newline added based"] # [doc = " on the parent [`ConsumeBinaryHunk`] configuration, also in unified diff format."] # [doc = " The `header` is the data used to produce `header_str`."] fn consume_binary_hunk (& mut self , header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > ; }
};
}
