// Generated macro for impl_223 (impl)
macro_rules! Depcrate_clipboardimpl_223 {
() => {
// Module: crate::clipboard
// Provides: {"impl_223"}
// Dependencies: {}
impl < T : AsRef < [u8] > > CopyToClipboard < T > { # [doc = " Construct a [`CopyToClipboard`] that writes content into the"] # [doc = " \"clipboard\" (or 'c') clipboard selection."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use crossterm::{execute, Command};"] # [doc = " use crossterm::clipboard::CopyToClipboard;"] # [doc = " execute!(std::io::stdout(), CopyToClipboard::to_clipboard_from(\"foo\"));"] # [doc = " ```"] pub fn to_clipboard_from (content : T) -> CopyToClipboard < T > { CopyToClipboard { content , destination : ClipboardSelection (vec ! [ClipboardType :: Clipboard]) , } } # [doc = " Construct a [`CopyToClipboard`] that writes content into the \"primary\""] # [doc = " (or 'p') clipboard selection."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use crossterm::execute;"] # [doc = " use crossterm::clipboard::CopyToClipboard;"] # [doc = " execute!(std::io::stdout(), CopyToClipboard::to_primary_from(\"foo\"));"] # [doc = " ```"] pub fn to_primary_from (content : T) -> CopyToClipboard < T > { CopyToClipboard { content , destination : ClipboardSelection (vec ! [ClipboardType :: Primary]) , } } }
};
}
