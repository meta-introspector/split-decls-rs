// Generated macro for impl_1775 (impl)
macro_rules! Depcrate_perfcnt_intel_descriptionimpl_1775 {
() => {
// Module: crate::perfcnt::intel::description
// Provides: {"impl_1775"}
// Dependencies: {}
impl fmt :: Debug for MSRIndex { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { MSRIndex :: None => write ! (f , "MSRIndex::None") , MSRIndex :: One (a) => write ! (f , "MSRIndex::One({})" , a) , MSRIndex :: Two (a , b) => write ! (f , "MSRIndex::Two({}, {})" , a , b) , } } }
};
}
