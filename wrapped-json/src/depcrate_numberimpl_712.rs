// Generated macro for impl_712 (impl)
macro_rules! Depcrate_numberimpl_712 {
() => {
// Module: crate::number
// Provides: {"impl_712"}
// Dependencies: {}
# [cfg (not (feature = "arbitrary_precision"))] impl PartialEq for N { fn eq (& self , other : & Self) -> bool { match (self , other) { (N :: PosInt (a) , N :: PosInt (b)) => a == b , (N :: NegInt (a) , N :: NegInt (b)) => a == b , (N :: Float (a) , N :: Float (b)) => a == b , _ => false , } } }
};
}
