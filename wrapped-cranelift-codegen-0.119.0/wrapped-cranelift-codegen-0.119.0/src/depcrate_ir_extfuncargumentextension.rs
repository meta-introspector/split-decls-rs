// Generated macro for ArgumentExtension (enum)
macro_rules! Depcrate_ir_extfuncArgumentExtension {
() => {
// Module: crate::ir::extfunc
// Provides: {"ArgumentExtension"}
// Dependencies: {}
# [doc = " Function argument extension options."] # [doc = ""] # [doc = " On some architectures, small integer function arguments and/or return values are extended to"] # [doc = " the width of a general-purpose register."] # [doc = ""] # [doc = " This attribute specifies how an argument or return value should be extended *if the platform"] # [doc = " and ABI require it*. Because the frontend (CLIF generator) does not know anything about the"] # [doc = " particulars of the target's ABI, and the CLIF should be platform-independent, these attributes"] # [doc = " specify *how* to extend (according to the signedness of the original program) rather than"] # [doc = " *whether* to extend."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum ArgumentExtension { # [doc = " No extension, high bits are indeterminate."] None , # [doc = " Unsigned extension: high bits in register are 0."] Uext , # [doc = " Signed extension: high bits in register replicate sign bit."] Sext , }
};
}
