// Generated macro for Message (enum)
macro_rules! Depcrate_formatMessage {
() => {
// Module: crate::format
// Provides: {"Message"}
// Dependencies: {}
# [doc = " A cargo message"] # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (tag = "reason" , rename_all = "kebab-case")] # [allow (clippy :: large_enum_variant)] pub enum Message < 'a > { # [doc = " Build completed, all further output should not be parsed"] BuildFinished (BuildFinished) , # [doc = " The compiler generated an artifact"] # [serde (borrow)] CompilerArtifact (Artifact < 'a >) , # [doc = " The compiler wants to display a message"] # [serde (borrow)] CompilerMessage (FromCompiler < 'a >) , # [doc = " A build script successfully executed."] # [serde (borrow)] BuildScriptExecuted (BuildScript < 'a >) , # [cfg (not (feature = "strict_unstable"))] # [doc (hidden)] # [serde (other)] Unknown , }
};
}
