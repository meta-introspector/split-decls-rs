// Generated macro for AddUsePosition (enum)
macro_rules! DepcrateAddUsePosition {
() => {
// Module: crate
// Provides: {"AddUsePosition"}
// Dependencies: {}
# [derive (Debug , Deserialize)] pub enum AddUsePosition { # [serde (rename = "start")] Start , # [serde (rename = "end")] End , # [serde (rename = "after_use_path")] AfterUsePath (String) , }
};
}
