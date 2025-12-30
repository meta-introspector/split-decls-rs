// Generated macro for impl_240 (impl)
macro_rules! Depcrate_generatedimpl_240 {
() => {
// Module: crate::generated
// Provides: {"impl_240"}
// Dependencies: {}
impl NEFilterControlVerdict { extern_methods ! (# [doc = " This class method returns a verdict indicating that the flow should be allowed to go through, and also"] # [doc = " tell the data provider whether to update its rules or not."] # [doc = ""] # [doc = " Parameter `updateRules`: YES if the control provider has updated the rules and wants to communicate that to the data provider"] # [doc = ""] # [doc = " Returns: The NEFilterControlVerdict object."] # [unsafe (method (allowVerdictWithUpdateRules :))] # [unsafe (method_family = none)] pub unsafe fn allowVerdictWithUpdateRules (update_rules : bool ,) -> Retained < NEFilterControlVerdict >; # [doc = " This class method returns a verdict indicating that the flow should be dropped, and also tell the data"] # [doc = " provider whether to update its rules or not."] # [doc = ""] # [doc = " Parameter `updateRules`: YES if the control provider has updated the rules and wants to communicate that to the data provider"] # [doc = ""] # [doc = " Returns: The NEFilterControlVerdict object."] # [unsafe (method (dropVerdictWithUpdateRules :))] # [unsafe (method_family = none)] pub unsafe fn dropVerdictWithUpdateRules (update_rules : bool ,) -> Retained < NEFilterControlVerdict >; # [doc = " This class method returns a verdict indicating that the flow should be handled by the data provider,"] # [doc = " and the rules needed by the data provider have been set."] # [doc = ""] # [doc = " Returns: The NEFilterControlVerdict object."] # [unsafe (method (updateRules))] # [unsafe (method_family = none)] pub unsafe fn updateRules () -> Retained < NEFilterControlVerdict >;) ; }
};
}
