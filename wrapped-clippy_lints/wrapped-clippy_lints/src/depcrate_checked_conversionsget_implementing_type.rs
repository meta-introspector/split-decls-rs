// Generated macro for get_implementing_type (function)
macro_rules! Depcrate_checked_conversionsget_implementing_type {
() => {
// Module: crate::checked_conversions
// Provides: {"get_implementing_type"}
// Dependencies: {}
# [doc = " Gets the type which implements the called function"] fn get_implementing_type (path : & QPath < '_ > , candidates : & [Symbol] , function : Symbol) -> Option < Symbol > { if let QPath :: TypeRelative (ty , path) = & path && path . ident . name == function && let TyKind :: Path (QPath :: Resolved (None , tp)) = & ty . kind && let [int] = tp . segments { candidates . iter () . find (| c | int . ident . name == * * c) . copied () } else { None } }
};
}
