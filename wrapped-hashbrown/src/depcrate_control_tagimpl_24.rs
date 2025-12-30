// Generated macro for impl_24 (impl)
macro_rules! Depcrate_control_tagimpl_24 {
() => {
// Module: crate::control::tag
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Debug for Tag { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_special () { if self . special_is_empty () { f . pad ("EMPTY") } else { f . pad ("DELETED") } } else { f . debug_tuple ("full") . field (& (self . 0 & 0x7F)) . finish () } } }
};
}
