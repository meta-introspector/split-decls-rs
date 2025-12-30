// Generated macro for impl_130 (impl)
macro_rules! Depcrate_encode_refimpl_130 {
() => {
// Module: crate::encode_ref
// Provides: {"impl_130"}
// Dependencies: {}
impl < T > ValueOrd for EncodeValueRef < '_ , T > where T : ValueOrd , { fn value_cmp (& self , other : & Self) -> Result < Ordering > { self . 0 . value_cmp (other . 0) } }
};
}
