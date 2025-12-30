// Generated macro for const_eval_discriminant_variant (function)
macro_rules! Depcrate_constevalconst_eval_discriminant_variant {
() => {
// Module: crate::consteval
// Provides: {"const_eval_discriminant_variant"}
// Dependencies: {}
pub (crate) fn const_eval_discriminant_variant < 'db > (db : & 'db dyn HirDatabase , variant_id : EnumVariantId ,) -> Result < i128 , ConstEvalError < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; let def = variant_id . into () ; let body = db . body (def) ; let loc = variant_id . lookup (db) ; if matches ! (body [body . body_expr] , Expr :: Missing) { let prev_idx = loc . index . checked_sub (1) ; let value = match prev_idx { Some (prev_idx) => { 1 + db . const_eval_discriminant (loc . parent . enum_variants (db) . variants [prev_idx as usize] . 0 ,) ? } _ => 0 , } ; return Ok (value) ; } let repr = db . enum_signature (loc . parent) . repr ; let is_signed = repr . and_then (| repr | repr . int) . is_none_or (| int | int . is_signed ()) ; let mir_body = db . monomorphized_mir_body (def , GenericArgs :: new_from_iter (interner , []) , db . trait_environment_for_body (def) ,) ? ; let c = interpret_mir (db , mir_body , false , None) ? . 0 ? ; let c = if is_signed { try_const_isize (db , & c) . unwrap () } else { try_const_usize (db , c) . unwrap () as i128 } ; Ok (c) }
};
}
