// Generated macro for ValueLabelAssignments (enum)
macro_rules! Depcrate_irValueLabelAssignments {
() => {
// Module: crate::ir
// Provides: {"ValueLabelAssignments"}
// Dependencies: {}
# [doc = " Value label assignments: label starts or value aliases."] # [derive (Debug , Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum ValueLabelAssignments { # [doc = " Original value labels assigned at transform."] Starts (alloc :: vec :: Vec < ValueLabelStart >) , # [doc = " A value alias to original value."] Alias { # [doc = " Source location when it is in effect"] from : RelSourceLoc , # [doc = " The label index."] value : Value , } , }
};
}
