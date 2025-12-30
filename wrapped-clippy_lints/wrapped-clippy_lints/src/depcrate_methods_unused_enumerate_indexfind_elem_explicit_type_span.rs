// Generated macro for find_elem_explicit_type_span (function)
macro_rules! Depcrate_methods_unused_enumerate_indexfind_elem_explicit_type_span {
() => {
// Module: crate::methods::unused_enumerate_index
// Provides: {"find_elem_explicit_type_span"}
// Dependencies: {}
# [doc = " Find the span of the explicit type of the element."] # [doc = ""] # [doc = " # Returns"] # [doc = " If the tuple argument:"] # [doc = " * Has no explicit type, returns `None`"] # [doc = " * Has an explicit tuple type with an implicit element type (`(usize, _)`), returns `None`"] # [doc = " * Has an explicit tuple type with an explicit element type (`(_, i32)`), returns the span for"] # [doc = "   the element type."] fn find_elem_explicit_type_span (fn_decl : & FnDecl < '_ >) -> Option < Span > { if let [tuple_ty] = fn_decl . inputs && let TyKind :: Tup ([_idx_ty , elem_ty]) = tuple_ty . kind && ! matches ! (elem_ty . kind , TyKind :: Err (..) | TyKind :: Infer (())) { Some (elem_ty . span) } else { None } }
};
}
