// Generated macro for LabelValueLoc (enum)
macro_rules! Depcrate_value_labelLabelValueLoc {
() => {
// Module: crate::value_label
// Provides: {"LabelValueLoc"}
// Dependencies: {}
# [doc = " The particular location for a value."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum LabelValueLoc { # [doc = " Register."] Reg (Reg) , # [doc = " Offset from the Canonical Frame Address (aka CFA)."] CFAOffset (i64) , }
};
}
