// Generated macro for map_constraints (function)
macro_rules! Depcrate_x86_constraintmap_constraints {
() => {
// Module: crate::x86::constraint
// Provides: {"map_constraints"}
// Dependencies: {}
pub fn map_constraints (imm_type : & String , imm_width : u32) -> Option < Constraint > { if imm_width > 0 { let max : i64 = 2i64 . pow (imm_width) ; return Some (Constraint :: Range (0 .. max)) ; } match imm_type . as_str () { "_MM_FROUND" => Some (Constraint :: Set (vec ! [4 , 8 , 9 , 10 , 11])) , "_MM_INDEX_SCALE" => Some (Constraint :: Set (vec ! [1 , 2 , 4 , 8])) , "_MM_CMPINT" => Some (Constraint :: Range (0 .. 8)) , "_MM_REDUCE" => Some (Constraint :: Range (0 .. 8)) , "_MM_FROUND_SAE" => Some (Constraint :: Equal (8)) , "_MM_MANTISSA_NORM" => Some (Constraint :: Range (0 .. 4)) , "_MM_MANTISSA_NORM_ENUM" => Some (Constraint :: Range (0 .. 4)) , "_MM_MANTISSA_SIGN" => Some (Constraint :: Range (0 .. 3)) , "_MM_PERM" => Some (Constraint :: Range (0 .. 256)) , "_MM_PERM_ENUM" => Some (Constraint :: Range (0 .. 256)) , "_MM_CMPINT_ENUM" => Some (Constraint :: Range (0 .. 8)) , "_MM_ROUND_MODE" => Some (Constraint :: Set (vec ! [0 , 0x2 , 0x4 , 0x6])) , "_CMP_" => Some (Constraint :: Range (0 .. 32)) , _ => None , } }
};
}
