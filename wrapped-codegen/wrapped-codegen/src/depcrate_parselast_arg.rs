// Generated macro for last_arg (function)
macro_rules! Depcrate_parselast_arg {
() => {
// Module: crate::parse
// Provides: {"last_arg"}
// Dependencies: {}
fn last_arg (params : & PathArguments) -> & syn :: Type { let data = match params { PathArguments :: AngleBracketed (data) => data , _ => panic ! ("expected at least 1 type argument here") , } ; match data . args . last () . expect ("expected at least 1 type argument here") { GenericArgument :: Type (ty) => ty , _ => panic ! ("expected at least 1 type argument here") , } }
};
}
