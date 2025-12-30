// Generated macro for macro_77 (macro)
macro_rules! Depcrate_scriptermacro_77 {
() => {
// Module: crate::scripter
// Provides: {"macro_77"}
// Dependencies: {}
actor ! { # [derive (Debug)] pub struct Scripter { # [doc = " The name of the product, for display"] # [arg (value_name = "NAME")] product_name : String = "Product" , # [doc = " The directory under lib/ where the manifest lives"] # [arg (value_name = "DIR")] rel_manifest_dir : String = "manifestlib" , # [doc = " The string to print after successful installation"] # [arg (value_name = "MESSAGE")] success_message : String = "Installed." , # [doc = " Places to look for legacy manifests to uninstall"] # [arg (value_name = "DIRS")] legacy_manifest_dirs : String = "" , # [doc = " The name of the output script"] # [arg (value_name = "FILE")] output_script : String = "install.sh" , } }
};
}
