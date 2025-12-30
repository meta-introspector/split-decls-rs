// Generated macro for AbiParam (struct)
macro_rules! Depcrate_ir_extfuncAbiParam {
() => {
// Module: crate::ir::extfunc
// Provides: {"AbiParam"}
// Dependencies: {}
# [doc = " Function parameter or return value descriptor."] # [doc = ""] # [doc = " This describes the value type being passed to or from a function along with flags that affect"] # [doc = " how the argument is passed."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct AbiParam { # [doc = " Type of the argument value."] pub value_type : Type , # [doc = " Special purpose of argument, or `Normal`."] pub purpose : ArgumentPurpose , # [doc = " Method for extending argument to a full register."] pub extension : ArgumentExtension , }
};
}
