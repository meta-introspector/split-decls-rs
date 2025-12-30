// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < KeySize > KeyInit for Rc4Core < KeySize > where KeySize : ArraySize , { fn new (key : & Key < KeySize >) -> Self { Self { state : Rc4State :: new (key) , key_size : Default :: default () , } } }
};
}
