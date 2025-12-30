// Generated macro for ClipboardSelection (struct)
macro_rules! Depcrate_clipboardClipboardSelection {
() => {
// Module: crate::clipboard
// Provides: {"ClipboardSelection"}
// Dependencies: {}
# [doc = " A sequence of clipboard types"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ClipboardSelection (# [doc = " An ordered list of clipboards which will be the destination for the copied selection."] # [doc = ""] # [doc = " Order matters due to implementations deviating from the"] # [doc = " [XTerm Control Sequences](https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Operating-System-Commands)"] # [doc = " reference. Some terminal emulators may only interpret the first character of this"] # [doc = " parameter. For differences, see"] # [doc = " [`CopyToClipboard` (Terminal Support)](struct.CopyToClipboard.html#terminal-support)."] pub Vec < ClipboardType > ,) ;
};
}
