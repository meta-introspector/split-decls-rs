// Generated macro for impl_4428 (impl)
macro_rules! Depcrate_manual_rotateimpl_4428 {
() => {
// Module: crate::manual_rotate
// Provides: {"impl_4428"}
// Dependencies: {}
impl Display for ShiftDirection { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { Self :: Left => "rotate_left" , Self :: Right => "rotate_right" , }) } }
};
}
