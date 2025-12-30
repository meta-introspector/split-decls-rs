// Generated macro for CargoMessage (enum)
macro_rules! Depcrate_core_build_steps_compileCargoMessage {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"CargoMessage"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (tag = "reason" , rename_all = "kebab-case")] pub enum CargoMessage < 'a > { CompilerArtifact { filenames : Vec < Cow < 'a , str > > , target : CargoTarget < 'a > } , BuildScriptExecuted , BuildFinished , }
};
}
