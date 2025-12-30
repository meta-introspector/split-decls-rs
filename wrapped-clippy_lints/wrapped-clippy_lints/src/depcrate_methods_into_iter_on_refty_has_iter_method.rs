// Generated macro for ty_has_iter_method (function)
macro_rules! Depcrate_methods_into_iter_on_refty_has_iter_method {
() => {
// Module: crate::methods::into_iter_on_ref
// Provides: {"ty_has_iter_method"}
// Dependencies: {}
fn ty_has_iter_method (cx : & LateContext < '_ > , self_ref_ty : Ty < '_ >) -> Option < (Symbol , & 'static str) > { has_iter_method (cx , self_ref_ty) . map (| ty_name | { let ty :: Ref (_ , _ , mutbl) = self_ref_ty . kind () else { unreachable ! () } ; let method_name = match mutbl { hir :: Mutability :: Not => "iter" , hir :: Mutability :: Mut => "iter_mut" , } ; (ty_name , method_name) }) }
};
}
