// Generated macro for ty_peel_refs (function)
macro_rules! Depcrate_matches_significant_drop_in_scrutineety_peel_refs {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"ty_peel_refs"}
// Dependencies: {}
fn ty_peel_refs (mut ty : Ty < '_ >) -> (Ty < '_ > , usize) { let mut n = 0 ; while let rustc_middle :: ty :: Ref (_ , new_ty , Mutability :: Not) = ty . kind () { ty = * new_ty ; n += 1 ; } (ty , n) }
};
}
