// Generated macro for impl_877 (impl)
macro_rules! Depcrate_ir_instructionsimpl_877 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_877"}
// Dependencies: {}
impl ValueTypeSet { # [doc = " Is `scalar` part of the base type set?"] # [doc = ""] # [doc = " Note that the base type set does not have to be included in the type set proper."] fn is_base_type (self , scalar : Type) -> bool { let l2b = u8 :: try_from (scalar . log2_lane_bits ()) . unwrap () ; if scalar . is_int () { self . ints . contains (l2b) } else if scalar . is_float () { self . floats . contains (l2b) } else { false } } # [doc = " Does `typ` belong to this set?"] pub fn contains (self , typ : Type) -> bool { if typ . is_dynamic_vector () { let l2l = u8 :: try_from (typ . log2_min_lane_count ()) . unwrap () ; self . dynamic_lanes . contains (l2l) && self . is_base_type (typ . lane_type ()) } else { let l2l = u8 :: try_from (typ . log2_lane_count ()) . unwrap () ; self . lanes . contains (l2l) && self . is_base_type (typ . lane_type ()) } } # [doc = " Get an example member of this type set."] # [doc = ""] # [doc = " This is used for error messages to avoid suggesting invalid types."] pub fn example (self) -> Type { let t = if self . ints . max () . unwrap_or (0) > 5 { types :: I32 } else if self . floats . max () . unwrap_or (0) > 5 { types :: F32 } else { types :: I8 } ; t . by (1 << self . lanes . min () . unwrap ()) . unwrap () } }
};
}
