// Generated macro for impl_24 (impl)
macro_rules! Depcrate_notifyimpl_24 {
() => {
// Module: crate::notify
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "std")] impl < N : fmt :: Debug , F > fmt :: Debug for TagWith < N , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct Ellipses ; impl fmt :: Debug for Ellipses { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("..") } } f . debug_struct ("TagWith") . field ("tag" , & Ellipses) . field ("inner" , & self . inner) . finish () } }
};
}
