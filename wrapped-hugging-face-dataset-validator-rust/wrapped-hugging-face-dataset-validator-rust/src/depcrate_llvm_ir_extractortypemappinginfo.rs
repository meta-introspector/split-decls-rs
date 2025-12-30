// Generated macro for TypeMappingInfo (struct)
macro_rules! Depcrate_llvm_ir_extractorTypeMappingInfo {
() => {
// Module: crate::llvm_ir_extractor
// Provides: {"TypeMappingInfo"}
// Dependencies: {}
# [doc = " Type mapping information between Rust and LLVM"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TypeMappingInfo { # [doc = " Original Rust type"] pub rust_type : String , # [doc = " Corresponding LLVM type"] pub llvm_type : String , # [doc = " Size in bytes"] pub size_bytes : u32 , # [doc = " Alignment requirements"] pub alignment : u32 , # [doc = " Whether the type is zero-sized"] pub is_zero_sized : bool , # [doc = " Generic parameters (if any)"] pub generic_params : Vec < String > , }
};
}
