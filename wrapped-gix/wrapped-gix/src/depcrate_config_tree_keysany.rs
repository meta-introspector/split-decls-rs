// Generated macro for Any (struct)
macro_rules! Depcrate_config_tree_keysAny {
() => {
// Module: crate::config::tree::keys
// Provides: {"Any"}
// Dependencies: {}
# [doc = " Implements a value without any constraints, i.e. a any value."] pub struct Any < T : Validate = validate :: All > { # [doc = " The key of the value in the git configuration."] pub name : & 'static str , # [doc = " The parent section of the key."] pub section : & 'static dyn Section , # [doc = " The subsection requirement to use."] pub subsection_requirement : Option < SubSectionRequirement > , # [doc = " A link to other resources that might be eligible as value."] pub link : Option < Link > , # [doc = " A note about this key."] pub note : Option < Note > , # [doc = " The way validation and transformation should happen."] validate : T , }
};
}
