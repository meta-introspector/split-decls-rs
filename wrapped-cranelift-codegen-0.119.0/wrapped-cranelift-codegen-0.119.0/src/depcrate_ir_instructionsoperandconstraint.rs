// Generated macro for OperandConstraint (enum)
macro_rules! Depcrate_ir_instructionsOperandConstraint {
() => {
// Module: crate::ir::instructions
// Provides: {"OperandConstraint"}
// Dependencies: {}
# [doc = " Operand constraints. This describes the value type constraints on a single `Value` operand."] enum OperandConstraint { # [doc = " This operand has a concrete value type."] Concrete (Type) , # [doc = " This operand can vary freely within the given type set."] # [doc = " The type set is identified by its index into the TYPE_SETS constant table."] Free (u8) , # [doc = " This operand is the same type as the controlling type variable."] Same , # [doc = " This operand is `ctrlType.lane_of()`."] LaneOf , # [doc = " This operand is `ctrlType.as_truthy()`."] AsTruthy , # [doc = " This operand is `ctrlType.half_width()`."] HalfWidth , # [doc = " This operand is `ctrlType.double_width()`."] DoubleWidth , # [doc = " This operand is `ctrlType.split_lanes()`."] SplitLanes , # [doc = " This operand is `ctrlType.merge_lanes()`."] MergeLanes , # [doc = " This operands is `ctrlType.dynamic_to_vector()`."] DynamicToVector , # [doc = " This operand is `ctrlType.narrower()`."] Narrower , # [doc = " This operand is `ctrlType.wider()`."] Wider , }
};
}
