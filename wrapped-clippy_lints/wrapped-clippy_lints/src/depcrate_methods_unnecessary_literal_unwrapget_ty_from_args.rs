// Generated macro for get_ty_from_args (function)
macro_rules! Depcrate_methods_unnecessary_literal_unwrapget_ty_from_args {
() => {
// Module: crate::methods::unnecessary_literal_unwrap
// Provides: {"get_ty_from_args"}
// Dependencies: {}
fn get_ty_from_args < 'a > (args : Option < & 'a [hir :: GenericArg < 'a >] > , index : usize) -> Option < & 'a hir :: Ty < 'a , AmbigArg > > { let args = args ? ; if args . len () <= index { return None ; } match args [index] { hir :: GenericArg :: Type (ty) => Some (ty) , _ => None , } }
};
}
