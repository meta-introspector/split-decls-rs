// Generated macro for impl_35 (impl)
macro_rules! Depcrate_eventimpl_35 {
() => {
// Module: crate::event
// Provides: {"impl_35"}
// Dependencies: {}
impl fmt :: Debug for Event { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Event") . field ("kind" , & self . kind) . field ("paths" , & self . paths) . field ("attr:tracker" , & self . tracker ()) . field ("attr:flag" , & self . flag ()) . field ("attr:info" , & self . info ()) . field ("attr:source" , & self . source ()) . finish () } }
};
}
