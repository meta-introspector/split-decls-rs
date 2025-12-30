// Generated macro for get_mutable_reference_type (function)
macro_rules! Depcrateget_mutable_reference_type {
() => {
// Module: crate
// Provides: {"get_mutable_reference_type"}
// Dependencies: {}
fn get_mutable_reference_type (arg : & syn :: FnArg) -> Option < & Type > { if let syn :: FnArg :: Typed (pat) = arg { if let syn :: Type :: Reference (refty) = & * pat . ty { if refty . mutability . is_some () { Some (& refty . elem) } else { None } } else { None } } else { None } }
};
}
