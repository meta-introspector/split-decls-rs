// Generated macro for impl_66 (impl)
macro_rules! Depcrate_trainimpl_66 {
() => {
// Module: crate::train
// Provides: {"impl_66"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsTrainCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , capitalize , | f | write ! (f , "-") , f) } }
};
}
