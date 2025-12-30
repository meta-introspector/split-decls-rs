// Generated macro for JsonMessage (enum)
macro_rules! Depcrate_flycheckJsonMessage {
() => {
// Module: crate::flycheck
// Provides: {"JsonMessage"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (untagged)] enum JsonMessage { Cargo (cargo_metadata :: Message) , Rustc (Diagnostic) , }
};
}
