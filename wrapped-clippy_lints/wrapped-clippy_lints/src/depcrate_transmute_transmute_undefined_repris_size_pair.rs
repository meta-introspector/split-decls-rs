// Generated macro for is_size_pair (function)
macro_rules! Depcrate_transmute_transmute_undefined_repris_size_pair {
() => {
// Module: crate::transmute::transmute_undefined_repr
// Provides: {"is_size_pair"}
// Dependencies: {}
fn is_size_pair (ty : Ty < '_ >) -> bool { if let ty :: Tuple (tys) = * ty . kind () && let [ty1 , ty2] = & * * tys { matches ! (ty1 . kind () , ty :: Int (IntTy :: Isize) | ty :: Uint (UintTy :: Usize)) && matches ! (ty2 . kind () , ty :: Int (IntTy :: Isize) | ty :: Uint (UintTy :: Usize)) } else { false } }
};
}
