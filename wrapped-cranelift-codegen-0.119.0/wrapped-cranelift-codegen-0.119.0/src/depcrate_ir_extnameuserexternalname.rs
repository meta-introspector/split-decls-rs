// Generated macro for UserExternalName (struct)
macro_rules! Depcrate_ir_extnameUserExternalName {
() => {
// Module: crate::ir::extname
// Provides: {"UserExternalName"}
// Dependencies: {}
# [doc = " An external name in a user-defined symbol table."] # [doc = ""] # [doc = " Cranelift does not interpret these numbers in any way, so they can represent arbitrary values."] # [derive (Debug , Clone , PartialEq , Eq , Hash , Default)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct UserExternalName { # [doc = " Arbitrary."] pub namespace : u32 , # [doc = " Arbitrary."] pub index : u32 , }
};
}
