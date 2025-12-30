// Generated macro for impl_401 (impl)
macro_rules! Depcrateimpl_401 {
() => {
// Module: crate
// Provides: {"impl_401"}
// Dependencies: {}
impl LengthCoder { pub fn new () -> Self { Self { choice : Default :: default () , low : Default :: default () , mid : Default :: default () , high : [0 ; HIGH_SYMBOLS] , } } pub fn reset (& mut self) { init_probs (& mut self . choice) ; for ele in self . low . iter_mut () { init_probs (ele) ; } for ele in self . mid . iter_mut () { init_probs (ele) ; } init_probs (& mut self . high) ; } }
};
}
