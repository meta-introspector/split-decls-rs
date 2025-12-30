// Generated macro for impl_131 (impl)
macro_rules! Depcrate_entry_modeimpl_131 {
() => {
// Module: crate::entry::mode
// Provides: {"impl_131"}
// Dependencies: {}
impl Change { # [doc = " Applies this change to `mode` and returns the changed one."] pub fn apply (self , mode : Mode) -> Mode { match self { Change :: Type { new_mode } => new_mode , Change :: ExecutableBit => match mode { Mode :: FILE => Mode :: FILE_EXECUTABLE , Mode :: FILE_EXECUTABLE => Mode :: FILE , _ => unreachable ! ("invalid mode change: can't flip executable bit of {mode:?}") , } , } } }
};
}
