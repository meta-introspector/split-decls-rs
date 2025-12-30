// Generated macro for impl_220 (impl)
macro_rules! Depcrate_clipboardimpl_220 {
() => {
// Module: crate::clipboard
// Provides: {"impl_220"}
// Dependencies: {}
impl FromStr for ClipboardSelection { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (ClipboardSelection (s . chars () . map (From :: < char > :: from) . collect () ,)) } }
};
}
