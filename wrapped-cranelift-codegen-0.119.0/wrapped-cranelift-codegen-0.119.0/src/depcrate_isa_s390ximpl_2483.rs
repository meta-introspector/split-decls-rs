// Generated macro for impl_2483 (impl)
macro_rules! Depcrate_isa_s390ximpl_2483 {
() => {
// Module: crate::isa::s390x
// Provides: {"impl_2483"}
// Dependencies: {}
impl fmt :: Display for S390xBackend { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MachBackend") . field ("name" , & self . name ()) . field ("triple" , & self . triple ()) . field ("flags" , & format ! ("{}" , self . flags ())) . finish () } }
};
}
