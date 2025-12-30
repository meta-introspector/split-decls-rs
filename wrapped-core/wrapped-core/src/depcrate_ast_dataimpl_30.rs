// Generated macro for impl_30 (impl)
macro_rules! Depcrate_ast_dataimpl_30 {
() => {
// Module: crate::ast::data
// Provides: {"impl_30"}
// Dependencies: {}
impl < V : FromVariant , F : FromField > Data < V , F > { # [doc = " Attempt to convert from a `syn::Data` instance."] pub fn try_from (body : & syn :: Data) -> Result < Self > { match * body { syn :: Data :: Enum (ref data) => { let mut errors = Error :: accumulator () ; let items = data . variants . iter () . filter_map (| v | errors . handle (FromVariant :: from_variant (v))) . collect () ; errors . finish_with (Data :: Enum (items)) } syn :: Data :: Struct (ref data) => Ok (Data :: Struct (Fields :: try_from (& data . fields) ?)) , syn :: Data :: Union (_) => Err (Error :: custom ("Unions are not supported")) , } } }
};
}
