// Generated macro for impl_540 (impl)
macro_rules! Depcrate_generatedimpl_540 {
() => {
// Module: crate::generated
// Provides: {"impl_540"}
// Dependencies: {}
impl NEOnDemandRuleEvaluateConnection { extern_methods ! (# [doc = " An array of NEEvaluateConnectionRule objects. Each NEEvaluateConnectionRule object is evaluated in order against the properties of the network connection being established."] # [unsafe (method (connectionRules))] # [unsafe (method_family = none)] pub unsafe fn connectionRules (& self) -> Option < Retained < NSArray < NEEvaluateConnectionRule >>>; # [doc = " Setter for [`connectionRules`][Self::connectionRules]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setConnectionRules :))] # [unsafe (method_family = none)] pub unsafe fn setConnectionRules (& self , connection_rules : Option <& NSArray < NEEvaluateConnectionRule >>,) ;) ; }
};
}
