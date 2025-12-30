// Generated macro for walk_ptrs_ty_depth (function)
macro_rules! Depcrate_tywalk_ptrs_ty_depth {
() => {
// Module: crate::ty
// Provides: {"walk_ptrs_ty_depth"}
// Dependencies: {}
# [doc = " Returns the base type for references and raw pointers, and count reference"] # [doc = " depth."] pub fn walk_ptrs_ty_depth (ty : Ty < '_ >) -> (Ty < '_ > , usize) { fn inner (ty : Ty < '_ > , depth : usize) -> (Ty < '_ > , usize) { match ty . kind () { ty :: Ref (_ , ty , _) => inner (* ty , depth + 1) , _ => (ty , depth) , } } inner (ty , 0) }
};
}
