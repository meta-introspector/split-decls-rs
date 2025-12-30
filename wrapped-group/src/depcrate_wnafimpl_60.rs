// Generated macro for impl_60 (impl)
macro_rules! Depcrate_wnafimpl_60 {
() => {
// Module: crate::wnaf
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , G : Group > Wnaf < usize , & 'a [G] , & 'a mut Vec < i64 > > { # [doc = " Constructs new space for the scalar representation while borrowing"] # [doc = " the computed window table, for sending the window table across threads."] pub fn shared (& self) -> Wnaf < usize , & 'a [G] , Vec < i64 > > { Wnaf { base : self . base , scalar : vec ! [] , window_size : self . window_size , } } }
};
}
