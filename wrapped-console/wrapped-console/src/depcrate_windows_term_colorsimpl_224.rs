// Generated macro for impl_224 (impl)
macro_rules! Depcrate_windows_term_colorsimpl_224 {
() => {
// Module: crate::windows_term::colors
// Provides: {"impl_224"}
// Dependencies: {}
impl FgBg { fn new (byte : u8) -> Option < Self > { match byte { b'3' => Some (Self :: Foreground) , b'4' => Some (Self :: Background) , _ => None , } } }
};
}
