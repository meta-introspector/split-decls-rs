// Generated macro for CompiledKeys (struct)
macro_rules! Depcrate_compiled_keysCompiledKeys {
() => {
// Module: crate::compiled_keys
// Provides: {"CompiledKeys"}
// Dependencies: {}
# [doc = " A helper struct to collect pubkeys compiled for a set of instructions"] # [derive (Default , Debug , Clone , PartialEq , Eq)] pub (crate) struct CompiledKeys { payer : Option < Address > , key_meta_map : BTreeMap < Address , CompiledKeyMeta > , }
};
}
