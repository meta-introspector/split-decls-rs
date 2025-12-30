// Generated macro for Signature (struct)
macro_rules! Depcrate_ir_extfuncSignature {
() => {
// Module: crate::ir::extfunc
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Function signature."] # [doc = ""] # [doc = " The function signature describes the types of formal parameters and return values along with"] # [doc = " other details that are needed to call a function correctly."] # [doc = ""] # [doc = " A signature can optionally include ISA-specific ABI information which specifies exactly how"] # [doc = " arguments and return values are passed."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Signature { # [doc = " The arguments passed to the function."] pub params : Vec < AbiParam > , # [doc = " Values returned from the function."] pub returns : Vec < AbiParam > , # [doc = " Calling convention."] pub call_conv : CallConv , }
};
}
