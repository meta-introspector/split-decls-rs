// Generated macro for ScriptWithExt (struct)
macro_rules! Depcrate_scriptScriptWithExt {
() => {
// Module: crate::script
// Provides: {"ScriptWithExt"}
// Dependencies: {}
# [doc = " An internal-use only pseudo-property that represents the values stored in"] # [doc = " the trie of the special data structure [`ScriptWithExtensionsProperty`]."] # [doc = ""] # [doc = " Note: The will assume a 12-bit layout. The 2 higher order bits in positions"] # [doc = " 11..10 will indicate how to deduce the Script value and Script_Extensions,"] # [doc = " and the lower 10 bits 9..0 indicate either the Script value or the index"] # [doc = " into the `extensions` structure."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_properties :: script))] # [repr (transparent)] # [doc (hidden)] # [allow (clippy :: exhaustive_structs)] pub struct ScriptWithExt (pub u16) ;
};
}
