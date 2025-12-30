// Generated macro for impl_608 (impl)
macro_rules! Depcrate_seimpl_608 {
() => {
// Module: crate::se
// Provides: {"impl_608"}
// Dependencies: {}
impl WriteResult { # [doc = " Returns `true` if indent should be written after the object (if configured) and `false` otherwise."] # [inline] pub fn allow_indent (& self) -> bool { matches ! (self , Self :: Element | Self :: Nothing) } # [doc = " Returns `true` if self is `Text` or `SensitiveText`."] # [inline] pub fn is_text (& self) -> bool { matches ! (self , Self :: Text | Self :: SensitiveText) } }
};
}
