// Generated macro for Criticality (enum)
macro_rules! Depcrate_policy_extensionCriticality {
() => {
// Module: crate::policy::extension
// Provides: {"Criticality"}
// Dependencies: {}
# [doc = " Represents different criticality states for an extension."] # [derive (Clone)] pub enum Criticality { # [doc = " The extension MUST be marked as critical."] Critical , # [doc = " The extension MAY be marked as critical."] Agnostic , # [doc = " The extension MUST NOT be marked as critical."] NonCritical , }
};
}
