// Generated macro for AuthContext (struct)
macro_rules! Depcrate_hooksAuthContext {
() => {
// Module: crate::hooks
// Provides: {"AuthContext"}
// Dependencies: {}
# [doc = " The context received by an authorizer hook."] # [doc = ""] # [doc = " See <https://sqlite.org/c3ref/set_authorizer.html> for more info."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct AuthContext < 'c > { # [doc = " The action to be authorized."] pub action : AuthAction < 'c > , # [doc = " The database name, if applicable."] pub database_name : Option < & 'c str > , # [doc = " The inner-most trigger or view responsible for the access attempt."] # [doc = " `None` if the access attempt was made by top-level SQL code."] pub accessor : Option < & 'c str > , }
};
}
