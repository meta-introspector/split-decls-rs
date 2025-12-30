// Generated macro for impl_326 (impl)
macro_rules! Depcrate_generatedimpl_326 {
() => {
// Module: crate::generated
// Provides: {"impl_326"}
// Dependencies: {}
impl NEFilterRule { extern_methods ! (# [doc = " Initialize a newly-allocated NEFilterRule object"] # [doc = ""] # [doc = " Parameter `networkRule`: A NENetworkRule object that defines the network traffic characteristics that this rule matches."] # [doc = ""] # [doc = " Parameter `action`: The action to take when this rule matches."] # [unsafe (method (initWithNetworkRule : action :))] # [unsafe (method_family = init)] pub unsafe fn initWithNetworkRule_action (this : Allocated < Self >, network_rule : & NENetworkRule , action : NEFilterAction ,) -> Retained < Self >; # [doc = " The NENetworkRule that defines the network traffic characteristics that this rule matches."] # [unsafe (method (networkRule))] # [unsafe (method_family = none)] pub unsafe fn networkRule (& self) -> Retained < NENetworkRule >; # [doc = " The action to take when this rule matches network traffic."] # [unsafe (method (action))] # [unsafe (method_family = none)] pub unsafe fn action (& self) -> NEFilterAction ;) ; }
};
}
