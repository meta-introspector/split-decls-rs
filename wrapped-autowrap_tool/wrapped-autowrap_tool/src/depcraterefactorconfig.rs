// Generated macro for RefactorConfig (struct)
macro_rules! DepcrateRefactorConfig {
() => {
// Module: crate
// Provides: {"RefactorConfig"}
// Dependencies: {}
# [derive (Deserialize)] pub struct RefactorConfig { pub refactor : RefactorMeta , # [serde (default)] pub splits : Vec < SplitRule > , pub export : ExportConfig , # [serde (default)] pub decl_refactoring : Option < DeclRefactoringConfig > , }
};
}
