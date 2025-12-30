// Generated macro for impl_69 (impl)
macro_rules! Depcrate_color_contextimpl_69 {
() => {
// Module: crate::color_context
// Provides: {"impl_69"}
// Dependencies: {}
impl ColorKind { pub fn to_change (& self , color : Color) -> Change { match self { Self :: Foreground => Change :: Foreground (color) , Self :: Background => Change :: Background (color) , } } }
};
}
