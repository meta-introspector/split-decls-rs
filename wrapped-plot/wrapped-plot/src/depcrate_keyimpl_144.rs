// Generated macro for impl_144 (impl)
macro_rules! Depcrate_keyimpl_144 {
() => {
// Module: crate::key
// Provides: {"impl_144"}
// Dependencies: {}
impl Set < Position > for Properties { # [doc = " Selects where to place the key"] # [doc = ""] # [doc = " **Note** By default, the key is placed `Inside(Vertical::Top, Horizontal::Right)`"] fn set (& mut self , position : Position) -> & mut Properties { self . position = Some (position) ; self } }
};
}
