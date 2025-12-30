// Generated macro for is_slice_of_primitives (function)
macro_rules! Depcrateis_slice_of_primitives {
() => {
// Module: crate
// Provides: {"is_slice_of_primitives"}
// Dependencies: {}
# [doc = " Returns `Option<String>` where String is a textual representation of the type encapsulated in"] # [doc = " the slice iff the given expression is a slice of primitives."] # [doc = ""] # [doc = " (As defined in the `is_recursively_primitive_type` function.) Returns `None` otherwise."] pub fn is_slice_of_primitives (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < String > { let expr_type = cx . typeck_results () . expr_ty_adjusted (expr) ; let expr_kind = expr_type . kind () ; let is_primitive = match expr_kind { rustc_ty :: Slice (element_type) => is_recursively_primitive_type (* element_type) , rustc_ty :: Ref (_ , inner_ty , _) if matches ! (inner_ty . kind () , & rustc_ty :: Slice (_)) => { if let rustc_ty :: Slice (element_type) = inner_ty . kind () { is_recursively_primitive_type (* element_type) } else { unreachable ! () } } , _ => false , } ; if is_primitive { match expr_type . peel_refs () . walk () . nth (1) . unwrap () . expect_ty () . kind () { rustc_ty :: Slice (..) => return Some ("slice" . into ()) , rustc_ty :: Array (..) => return Some ("array" . into ()) , rustc_ty :: Tuple (..) => return Some ("tuple" . into ()) , _ => { let refs_peeled = expr_type . peel_refs () ; return Some (refs_peeled . walk () . last () . unwrap () . to_string ()) ; } , } } None }
};
}
