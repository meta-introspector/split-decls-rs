// Generated macro for impl_39 (impl)
macro_rules! Depcrate_arm_json_parserimpl_39 {
() => {
// Module: crate::arm::json_parser
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " ARM-specific"] impl TryFrom < ArgPrep > for Constraint { type Error = () ; fn try_from (prep : ArgPrep) -> Result < Self , Self :: Error > { let parsed_ints = match prep { ArgPrep :: Immediate { min , max } => Ok ((min , max)) , _ => Err (()) , } ; if let Ok ((min , max)) = parsed_ints { if min == max { Ok (Constraint :: Equal (min)) } else { Ok (Constraint :: Range (min .. max + 1)) } } else { Err (()) } } }
};
}
