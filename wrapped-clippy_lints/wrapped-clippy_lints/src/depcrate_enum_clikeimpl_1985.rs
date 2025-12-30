// Generated macro for impl_1985 (impl)
macro_rules! Depcrate_enum_clikeimpl_1985 {
() => {
// Module: crate::enum_clike
// Provides: {"impl_1985"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnportableVariant { # [expect (clippy :: cast_possible_wrap)] fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if cx . tcx . data_layout . pointer_size () . bits () != 64 { return ; } if let ItemKind :: Enum (_ , _ , def) = & item . kind { for var in def . variants { if let Some (anon_const) = & var . disr_expr { let def_id = cx . tcx . hir_body_owner_def_id (anon_const . body) ; let mut ty = cx . tcx . type_of (def_id . to_def_id ()) . instantiate_identity () ; let constant = cx . tcx . const_eval_poly (def_id . to_def_id ()) . ok () ; if let Some (Constant :: Int (val)) = constant . and_then (| c | mir_to_const (cx . tcx , c , ty)) { if let ty :: Adt (adt , _) = ty . kind () && adt . is_enum () { ty = adt . repr () . discr_type () . to_ty (cx . tcx) ; } match ty . kind () { ty :: Int (IntTy :: Isize) => { let val = ((val as i128) << 64) >> 64 ; if i32 :: try_from (val) . is_ok () { continue ; } } , ty :: Uint (UintTy :: Usize) if val > u128 :: from (u32 :: MAX) => { } , _ => continue , } span_lint (cx , ENUM_CLIKE_UNPORTABLE_VARIANT , var . span , "C-like enum variant discriminant is not portable to 32-bit targets" ,) ; } } } } } }
};
}
