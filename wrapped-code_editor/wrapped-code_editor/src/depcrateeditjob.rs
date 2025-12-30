// Generated macro for EditJob (enum)
macro_rules! DepcrateEditJob {
() => {
// Module: crate
// Provides: {"EditJob"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (tag = "type")] pub enum EditJob { AddUse (AddUseDetails) , RemoveFunction (RemoveFunctionDetails) , ReplaceExpression (ReplaceExpressionDetails) , AddFunction (AddFunctionDetails) , AddItem (AddItemDetails) , ReplaceFileContent (ReplaceFileContentDetails) , ReplaceFileContentFromFile (ReplaceFileContentFromFileDetails) , }
};
}
