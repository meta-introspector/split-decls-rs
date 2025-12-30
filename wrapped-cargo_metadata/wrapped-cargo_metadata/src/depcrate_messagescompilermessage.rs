// Generated macro for CompilerMessage (struct)
macro_rules! Depcrate_messagesCompilerMessage {
() => {
// Module: crate::messages
// Provides: {"CompilerMessage"}
// Dependencies: {}
# [doc = " Message left by the compiler"] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct CompilerMessage { # [doc = " The package this message belongs to"] pub package_id : PackageId , # [doc = " The target this message is aimed at"] pub target : Target , # [doc = " The message the compiler sent."] pub message : Diagnostic , }
};
}
