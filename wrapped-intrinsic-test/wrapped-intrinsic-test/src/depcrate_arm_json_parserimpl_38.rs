// Generated macro for impl_38 (impl)
macro_rules! Depcrate_arm_json_parserimpl_38 {
() => {
// Module: crate::arm::json_parser
// Provides: {"impl_38"}
// Dependencies: {}
impl TryFrom < Value > for ArgPrep { type Error = serde_json :: Error ; fn try_from (value : Value) -> Result < Self , Self :: Error > { serde_json :: from_value (value) } }
};
}
