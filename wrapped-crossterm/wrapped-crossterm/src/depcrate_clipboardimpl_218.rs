// Generated macro for impl_218 (impl)
macro_rules! Depcrate_clipboardimpl_218 {
() => {
// Module: crate::clipboard
// Provides: {"impl_218"}
// Dependencies: {}
impl ClipboardSelection { # [doc = " Returns a String corresponsing to the \"Pc\" parameter of the OSC52"] # [doc = " sequence."] fn to_osc52_pc (& self) -> String { self . 0 . iter () . map (Into :: < char > :: into) . collect () } }
};
}
