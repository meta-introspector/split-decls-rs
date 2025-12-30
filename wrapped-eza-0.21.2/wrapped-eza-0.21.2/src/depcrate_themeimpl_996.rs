// Generated macro for impl_996 (impl)
macro_rules! Depcrate_themeimpl_996 {
() => {
// Module: crate::theme
// Provides: {"impl_996"}
// Dependencies: {}
impl < A , B > FileStyle for (A , B) where A : FileStyle , B : FileStyle , { fn get_style (& self , file : & File < '_ > , theme : & Theme) -> Option < Style > { self . 0 . get_style (file , theme) . or_else (| | self . 1 . get_style (file , theme)) } }
};
}
