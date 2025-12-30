// Generated macro for impl_172 (impl)
macro_rules! Depcrate_squeueimpl_172 {
() => {
// Module: crate::squeue
// Provides: {"impl_172"}
// Dependencies: {}
impl Debug for Entry { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry") . field ("op_code" , & self . 0 . opcode) . field ("flags" , & self . 0 . flags) . field ("user_data" , & self . 0 . user_data) . finish () } }
};
}
