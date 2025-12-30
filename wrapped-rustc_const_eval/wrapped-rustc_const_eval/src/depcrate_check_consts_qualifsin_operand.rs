// Generated macro for in_operand (function)
macro_rules! Depcrate_check_consts_qualifsin_operand {
() => {
// Module: crate::check_consts::qualifs
// Provides: {"in_operand"}
// Dependencies: {}
# [doc = " Returns `true` if this `Operand` contains qualif `Q`."] pub fn in_operand < 'tcx , Q , F > (cx : & ConstCx < '_ , 'tcx > , in_local : & mut F , operand : & Operand < 'tcx > ,) -> bool where Q : Qualif , F : FnMut (Local) -> bool , { let constant = match operand { Operand :: Copy (place) | Operand :: Move (place) => { return in_place :: < Q , _ > (cx , in_local , place . as_ref ()) ; } Operand :: Constant (c) => c , } ; let uneval = match constant . const_ { Const :: Ty (_ , ct) if matches ! (ct . kind () , ty :: ConstKind :: Param (_) | ty :: ConstKind :: Error (_) | ty :: ConstKind :: Value (_)) => { None } Const :: Ty (_ , c) => { bug ! ("expected ConstKind::Param or ConstKind::Value here, found {:?}" , c) } Const :: Unevaluated (uv , _) => Some (uv) , Const :: Val (..) => None , } ; if let Some (mir :: UnevaluatedConst { def , args : _ , promoted }) = uneval { assert ! (promoted . is_none () || Q :: ALLOW_PROMOTED) ; if promoted . is_none () && cx . tcx . trait_of_assoc (def) . is_none () { let qualifs = cx . tcx . at (constant . span) . mir_const_qualif (def) ; if ! Q :: in_qualifs (& qualifs) { return false ; } } } Q :: in_any_value_of_ty (cx , constant . const_ . ty ()) }
};
}
