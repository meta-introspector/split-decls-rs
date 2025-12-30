// Generated macro for ResolvedConstraint (enum)
macro_rules! Depcrate_ir_instructionsResolvedConstraint {
() => {
// Module: crate::ir::instructions
// Provides: {"ResolvedConstraint"}
// Dependencies: {}
# [doc = " The type constraint on a value argument once the controlling type variable is known."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ResolvedConstraint { # [doc = " The operand is bound to a known type."] Bound (Type) , # [doc = " The operand type can vary freely within the given set."] Free (ValueTypeSet) , }
};
}
