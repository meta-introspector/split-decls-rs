// Generated macro for has_lifetime (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedhas_lifetime {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"has_lifetime"}
// Dependencies: {}
fn has_lifetime (ty : Ty < '_ >) -> bool { ty . walk () . any (| t | matches ! (t . kind () , GenericArgKind :: Lifetime (_))) }
};
}
