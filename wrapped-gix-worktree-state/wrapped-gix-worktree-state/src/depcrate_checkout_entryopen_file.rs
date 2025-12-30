// Generated macro for open_file (function)
macro_rules! Depcrate_checkout_entryopen_file {
() => {
// Module: crate::checkout::entry
// Provides: {"open_file"}
// Dependencies: {}
pub (crate) fn open_file (path : & Path , destination_is_initially_empty : bool , overwrite_existing : bool , fs_supports_executable_bit : bool , entry_mode : gix_index :: entry :: Mode ,) -> std :: io :: Result < (std :: fs :: File , bool) > { # [cfg_attr (windows , allow (unused_mut))] let mut options = open_options (path , destination_is_initially_empty , overwrite_existing) ; let needs_executable_bit = fs_supports_executable_bit && entry_mode == gix_index :: entry :: Mode :: FILE_EXECUTABLE ; # [cfg (unix)] let set_executable_after_creation = if needs_executable_bit && destination_is_initially_empty { use std :: os :: unix :: fs :: OpenOptionsExt ; options . mode (0o777) ; false } else { needs_executable_bit } ; # [cfg (windows)] let set_executable_after_creation = needs_executable_bit ; try_op_or_unlink (path , overwrite_existing , | p | options . open (p)) . map (| f | (f , set_executable_after_creation)) }
};
}
