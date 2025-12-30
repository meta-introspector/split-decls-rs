// Generated macro for create_informational_target_machine (function)
macro_rules! Depcrate_back_writecreate_informational_target_machine {
() => {
// Module: crate::back::write
// Provides: {"create_informational_target_machine"}
// Dependencies: {}
pub (crate) fn create_informational_target_machine (sess : & Session , only_base_features : bool ,) -> OwnedTargetMachine { let config = TargetMachineFactoryConfig { split_dwarf_file : None , output_obj_file : None } ; let features = llvm_util :: global_llvm_features (sess , false , only_base_features) ; target_machine_factory (sess , config :: OptLevel :: No , & features) (config) . unwrap_or_else (| err | llvm_err (sess . dcx () , err)) }
};
}
