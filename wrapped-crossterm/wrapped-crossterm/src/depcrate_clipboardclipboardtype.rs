// Generated macro for ClipboardType (enum)
macro_rules! Depcrate_clipboardClipboardType {
() => {
// Module: crate::clipboard
// Provides: {"ClipboardType"}
// Dependencies: {}
# [doc = " Different clipboard types"] # [doc = ""] # [doc = " Some operating systems and desktop environments support multiple buffers"] # [doc = " for copy/cut/paste. Their details differ between operating systems."] # [doc = " See <https://specifications.freedesktop.org/clipboard-spec/latest/>"] # [doc = " for a detailed survey of supported types based on the X window system."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum ClipboardType { # [doc = " Default clipboard when using Ctrl+C or Ctrl+V"] Clipboard , # [doc = " Clipboard on Linux/X/Wayland when using selection and middle mouse button"] Primary , # [doc = " Other clipboard type not explicitly supported by crossterm"] # [doc = ""] # [doc = " See"] # [doc = " [XTerm Control Sequences](https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands)"] # [doc = " for potential values."] # [doc = ""] # [doc = " Note that support for these in terminal emulators is very limited."] Other (char) , }
};
}
