// Generated macro for impl_748 (impl)
macro_rules! Depcrate_generatedimpl_748 {
() => {
// Module: crate::generated
// Provides: {"impl_748"}
// Dependencies: {}
impl NWPath { extern_methods ! (# [doc = " The evaluated NWPathStatus of the NWPath."] # [deprecated = "Use `nw_path_get_status` in Network framework instead, see deprecation notice in <NetworkExtension/NWPath.h>"] # [unsafe (method (status))] # [unsafe (method_family = none)] pub unsafe fn status (& self) -> NWPathStatus ; # [doc = " Returns YES if the path is considered expensive, as when using a cellular data plan."] # [deprecated = "Use `nw_path_is_expensive` in Network framework instead, see deprecation notice in <NetworkExtension/NWPath.h>"] # [unsafe (method (isExpensive))] # [unsafe (method_family = none)] pub unsafe fn isExpensive (& self) -> bool ; # [doc = " Returns YES if the path is considered constrained, as when it is in save data mode."] # [deprecated = "Use `nw_path_is_constrained` in Network framework instead, see deprecation notice in <NetworkExtension/NWPath.h>"] # [unsafe (method (isConstrained))] # [unsafe (method_family = none)] pub unsafe fn isConstrained (& self) -> bool ; # [doc = " Parameter `path`: An NWPath object to compare."] # [doc = ""] # [doc = " Returns: YES if the two path objects have the same content, NO otherwise."] # [deprecated = "Use `nw_path_is_equal` in Network framework instead, see deprecation notice in <NetworkExtension/NWPath.h>"] # [unsafe (method (isEqualToPath :))] # [unsafe (method_family = none)] pub unsafe fn isEqualToPath (& self , path : & NWPath) -> bool ;) ; }
};
}
