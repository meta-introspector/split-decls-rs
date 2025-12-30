// Generated macro for impl_8 (impl)
macro_rules! Depcrate_bufimpl_8 {
() => {
// Module: crate::buf
// Provides: {"impl_8"}
// Dependencies: {}
impl < const SIZE : usize > fmt :: Debug for WriteBuffer < SIZE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DisplayBuffer") . field ("buf" , & self . as_str ()) . field ("remaining_capacity" , & self . remaining_capacity ()) . finish () } }
};
}
