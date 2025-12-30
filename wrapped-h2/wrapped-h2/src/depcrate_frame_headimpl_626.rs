// Generated macro for impl_626 (impl)
macro_rules! Depcrate_frame_headimpl_626 {
() => {
// Module: crate::frame::head
// Provides: {"impl_626"}
// Dependencies: {}
impl Kind { pub fn new (byte : u8) -> Kind { match byte { 0 => Kind :: Data , 1 => Kind :: Headers , 2 => Kind :: Priority , 3 => Kind :: Reset , 4 => Kind :: Settings , 5 => Kind :: PushPromise , 6 => Kind :: Ping , 7 => Kind :: GoAway , 8 => Kind :: WindowUpdate , 9 => Kind :: Continuation , _ => Kind :: Unknown , } } }
};
}
