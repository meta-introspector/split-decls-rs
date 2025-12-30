// Generated macro for find_output_type (function)
macro_rules! Depcrate_item_implfind_output_type {
() => {
// Module: crate::item_impl
// Provides: {"find_output_type"}
// Dependencies: {}
fn find_output_type (item_impl : & ItemImpl) -> Result < & Type > { for item in & item_impl . items { if let ImplItem :: Type (t) = item { if t . ident == "Output" { return Ok (& t . ty) ; } } } bail ! (_ , "cannot find associate type `Output`") ; }
};
}
