// Generated macro for impl_27 (impl)
macro_rules! Depcrate_policy_extensionimpl_27 {
() => {
// Module: crate::policy::extension
// Provides: {"impl_27"}
// Dependencies: {}
impl Criticality { pub (crate) fn permits (& self , critical : bool) -> bool { match (self , critical) { (Criticality :: Critical , true) => true , (Criticality :: Critical , false) => false , (Criticality :: Agnostic , _) => true , (Criticality :: NonCritical , true) => false , (Criticality :: NonCritical , false) => true , } } }
};
}
