// Generated macro for ValueLabelStart (struct)
macro_rules! Depcrate_irValueLabelStart {
() => {
// Module: crate::ir
// Provides: {"ValueLabelStart"}
// Dependencies: {}
# [doc = " A label of a Value."] # [derive (Debug , Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct ValueLabelStart { # [doc = " Source location when it is in effect"] pub from : RelSourceLoc , # [doc = " The label index."] pub label : ValueLabel , }
};
}
