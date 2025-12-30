// Generated macro for FromCompiler (struct)
macro_rules! Depcrate_formatFromCompiler {
() => {
// Module: crate::format
// Provides: {"FromCompiler"}
// Dependencies: {}
# [doc = " Message left by the compiler"] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [cfg_attr (feature = "strict_unstable" , serde (deny_unknown_fields))] # [non_exhaustive] pub struct FromCompiler < 'a > { # [doc = " The workspace member this message belongs to"] # [serde (borrow)] pub package_id : WorkspaceMember < 'a > , # [doc = " The full path to the artifact's manifest"] # [serde (borrow)] pub manifest_path : Option < CowPath < 'a > > , # [doc = " The target this message is aimed at"] # [serde (borrow)] pub target : Target < 'a > , # [doc = " The message the compiler sent."] # [serde (borrow)] pub message : diagnostic :: Diagnostic < 'a > , }
};
}
