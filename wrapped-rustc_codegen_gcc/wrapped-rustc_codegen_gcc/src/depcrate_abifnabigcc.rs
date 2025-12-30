// Generated macro for FnAbiGcc (struct)
macro_rules! Depcrate_abiFnAbiGcc {
() => {
// Module: crate::abi
// Provides: {"FnAbiGcc"}
// Dependencies: {}
pub struct FnAbiGcc < 'gcc > { pub return_type : Type < 'gcc > , pub arguments_type : Vec < Type < 'gcc > > , pub is_c_variadic : bool , pub on_stack_param_indices : FxHashSet < usize > , # [cfg (feature = "master")] pub fn_attributes : Vec < FnAttribute < 'gcc > > , }
};
}
