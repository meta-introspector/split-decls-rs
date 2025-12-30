// Generated macro for is_allowed_vec_method (function)
macro_rules! Depcrate_vecis_allowed_vec_method {
() => {
// Module: crate::vec
// Provides: {"is_allowed_vec_method"}
// Dependencies: {}
# [doc = " Checks if the given expression is a method call to a `Vec` method"] # [doc = " that also exists on slices. If this returns true, it means that"] # [doc = " this expression does not actually require a `Vec` and could just work with an array."] pub fn is_allowed_vec_method (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { if let ExprKind :: MethodCall (path , _ , [] , _) = e . kind { matches ! (path . ident . name , sym :: as_ptr | sym :: is_empty | sym :: len) } else { is_trait_method (cx , e , sym :: IntoIterator) } }
};
}
