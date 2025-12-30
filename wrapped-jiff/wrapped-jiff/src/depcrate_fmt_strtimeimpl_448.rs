// Generated macro for impl_448 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_448 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_448"}
// Dependencies: {}
impl < 'f > core :: fmt :: Debug for Display < 'f > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Display") . field ("fmt" , & escape :: Bytes (self . fmt)) . field ("tm" , & self . tm) . finish () } }
};
}
