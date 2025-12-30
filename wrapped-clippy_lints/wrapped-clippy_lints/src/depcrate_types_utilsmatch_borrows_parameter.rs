// Generated macro for match_borrows_parameter (function)
macro_rules! Depcrate_types_utilsmatch_borrows_parameter {
() => {
// Module: crate::types::utils
// Provides: {"match_borrows_parameter"}
// Dependencies: {}
pub (super) fn match_borrows_parameter (_cx : & LateContext < '_ > , qpath : & QPath < '_ >) -> Option < Span > { let last = last_path_segment (qpath) ; if let Some (params) = last . args && params . parenthesized == GenericArgsParentheses :: No && let Some (ty) = params . args . iter () . find_map (| arg | match arg { GenericArg :: Type (ty) => Some (ty) , _ => None , }) && let TyKind :: Ref (..) = ty . kind { return Some (ty . span) ; } None }
};
}
