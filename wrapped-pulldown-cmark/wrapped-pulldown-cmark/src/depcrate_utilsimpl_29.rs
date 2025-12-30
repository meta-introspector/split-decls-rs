// Generated macro for impl_29 (impl)
macro_rules! Depcrate_utilsimpl_29 {
() => {
// Module: crate::utils
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , I > TextMergeWithOffset < 'a , I > where I : Iterator < Item = (Event < 'a > , Range < usize >) > , { pub fn new (iter : I) -> Self { Self { iter , last_event : None , } } }
};
}
