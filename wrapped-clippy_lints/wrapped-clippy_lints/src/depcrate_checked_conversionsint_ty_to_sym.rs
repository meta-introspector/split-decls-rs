// Generated macro for int_ty_to_sym (function)
macro_rules! Depcrate_checked_conversionsint_ty_to_sym {
() => {
// Module: crate::checked_conversions
// Provides: {"int_ty_to_sym"}
// Dependencies: {}
# [doc = " Gets the type as a string, if it is a supported integer"] fn int_ty_to_sym (path : & QPath < '_ >) -> Option < Symbol > { if let QPath :: Resolved (_ , path) = * path && let [ty] = path . segments { INTS . iter () . find (| c | ty . ident . name == * * c) . copied () } else { None } }
};
}
