// Generated macro for first_arg (function)
macro_rules! Depcrate_parsefirst_arg {
() => {
// Module: crate::parse
// Provides: {"first_arg"}
// Dependencies: {}
fn first_arg (params : & PathArguments) -> & syn :: Type { let data = match params { PathArguments :: AngleBracketed (data) => data , _ => panic ! ("expected at least 1 type argument here") , } ; match data . args . first () . expect ("expected at least 1 type argument here") { GenericArgument :: Type (ty) => ty , _ => panic ! ("expected at least 1 type argument here") , } }
};
}
