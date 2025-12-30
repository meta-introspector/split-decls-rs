// Generated macro for ValueLocRange (struct)
macro_rules! Depcrate_value_labelValueLocRange {
() => {
// Module: crate::value_label
// Provides: {"ValueLocRange"}
// Dependencies: {}
# [doc = " Value location range."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct ValueLocRange { # [doc = " The ValueLoc containing a ValueLabel during this range."] pub loc : LabelValueLoc , # [doc = " The start of the range. It is an offset in the generated code."] pub start : u32 , # [doc = " The end of the range. It is an offset in the generated code."] pub end : u32 , }
};
}
