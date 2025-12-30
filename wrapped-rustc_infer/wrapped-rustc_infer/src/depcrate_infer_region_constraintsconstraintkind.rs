// Generated macro for ConstraintKind (enum)
macro_rules! Depcrate_infer_region_constraintsConstraintKind {
() => {
// Module: crate::infer::region_constraints
// Provides: {"ConstraintKind"}
// Dependencies: {}
# [doc = " Represents a constraint that influences the inference process."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum ConstraintKind { # [doc = " A region variable is a subregion of another."] VarSubVar , # [doc = " A concrete region is a subregion of region variable."] RegSubVar , # [doc = " A region variable is a subregion of a concrete region. This does not"] # [doc = " directly affect inference, but instead is checked after"] # [doc = " inference is complete."] VarSubReg , # [doc = " A constraint where neither side is a variable. This does not"] # [doc = " directly affect inference, but instead is checked after"] # [doc = " inference is complete."] RegSubReg , }
};
}
