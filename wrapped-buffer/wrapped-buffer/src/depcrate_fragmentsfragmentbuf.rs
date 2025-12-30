// Generated macro for FragmentBuf (struct)
macro_rules! Depcrate_fragmentsFragmentBuf {
() => {
// Module: crate::fragments
// Provides: {"FragmentBuf"}
// Dependencies: {}
struct FragmentBuf < 'sval , T : ? Sized + Fragment > { # [cfg (not (feature = "alloc"))] value : & 'sval T , # [cfg (feature = "alloc")] value : Cow < 'sval , T > , }
};
}
