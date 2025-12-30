// Generated macro for is_safe_due_to_smaller_source_type (function)
macro_rules! Depcrate_operators_arithmetic_side_effectsis_safe_due_to_smaller_source_type {
() => {
// Module: crate::operators::arithmetic_side_effects
// Provides: {"is_safe_due_to_smaller_source_type"}
// Dependencies: {}
# [doc = " If one side is a literal it is possible to evaluate overflows as long as the other side has a"] # [doc = " smaller type. `0` and `1` suffixes indicate different sides."] # [doc = ""] # [doc = " For example, `1000u64 + u64::from(some_runtime_variable_of_type_u8)`."] fn is_safe_due_to_smaller_source_type (cx : & LateContext < '_ > , op : hir :: BinOpKind , (expr0 , ty0) : (& hir :: Expr < '_ > , Ty < '_ >) , expr1 : & hir :: Expr < '_ > ,) -> bool { let Some (num0) = literal_integer (cx , expr0) else { return false ; } ; let Some (orig_ty1) = find_original_primitive_ty (cx , expr1) else { return false ; } ; let Some (num1) = max_int_num (orig_ty1) else { return false ; } ; let Some (rslt) = (match op { hir :: BinOpKind :: Add => num0 . checked_add (num1) , hir :: BinOpKind :: Mul => num0 . checked_mul (num1) , _ => None , }) else { return false ; } ; match ty0 . peel_refs () . kind () { ty :: Uint (UintTy :: U16) => u16 :: try_from (rslt) . is_ok () , ty :: Uint (UintTy :: U32) => u32 :: try_from (rslt) . is_ok () , ty :: Uint (UintTy :: U64) => u64 :: try_from (rslt) . is_ok () , ty :: Uint (UintTy :: U128) => true , ty :: Uint (UintTy :: Usize) => usize :: try_from (rslt) . is_ok () , _ => false , } }
};
}
