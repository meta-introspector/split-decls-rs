// Generated macro for AssistResolveStrategy (enum)
macro_rules! Depcrate_assistsAssistResolveStrategy {
() => {
// Module: crate::assists
// Provides: {"AssistResolveStrategy"}
// Dependencies: {}
# [doc = " A way to control how many assist to resolve during the assist resolution."] # [doc = " When an assist is resolved, its edits are calculated that might be costly to always do by default."] # [derive (Debug)] pub enum AssistResolveStrategy { # [doc = " No assists should be resolved."] None , # [doc = " All assists should be resolved."] All , # [doc = " Only a certain assist should be resolved."] Single (SingleResolve) , }
};
}
