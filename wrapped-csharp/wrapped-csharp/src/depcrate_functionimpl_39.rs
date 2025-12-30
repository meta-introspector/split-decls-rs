// Generated macro for impl_39 (impl)
macro_rules! Depcrate_functionimpl_39 {
() => {
// Module: crate::function
// Provides: {"impl_39"}
// Dependencies: {}
impl ResourceInfo { # [doc = " Returns the name of the exported implementation of this resource."] # [doc = ""] # [doc = " The result is only valid if the resource is actually being exported by the world."] fn export_impl_name (& self) -> String { format ! ("{}Impl.{}" , CSharp :: get_class_name_from_qualified_name (& self . module) . 1 . strip_prefix ("I") . unwrap () . to_upper_camel_case () , self . name . to_upper_camel_case ()) } }
};
}
