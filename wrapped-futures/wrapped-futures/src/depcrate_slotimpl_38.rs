// Generated macro for impl_38 (impl)
macro_rules! Depcrate_slotimpl_38 {
() => {
// Module: crate::slot
// Provides: {"impl_38"}
// Dependencies: {}
impl State { fn flag (& self , f : usize) -> bool { self . 0 & f != 0 } fn set_flag (& self , f : usize , val : bool) -> State { State (if val { self . 0 | f } else { self . 0 & ! f }) } fn token (& self) -> usize { self . 0 >> STATE_BITS } fn set_token (& self , gen : usize) -> State { State ((gen << STATE_BITS) | (self . 0 & STATE_MASK)) } }
};
}
