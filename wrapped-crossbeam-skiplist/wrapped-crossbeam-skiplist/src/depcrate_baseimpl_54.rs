// Generated macro for impl_54 (impl)
macro_rules! Depcrate_baseimpl_54 {
() => {
// Module: crate::base
// Provides: {"impl_54"}
// Dependencies: {}
impl < K , V > fmt :: Debug for RefIter < '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("RefIter") ; match & self . head { None => d . field ("head" , & None :: < (& K , & V) >) , Some (e) => d . field ("head" , & (e . key () , e . value ())) , } ; match & self . tail { None => d . field ("tail" , & None :: < (& K , & V) >) , Some (e) => d . field ("tail" , & (e . key () , e . value ())) , } ; d . finish () } }
};
}
