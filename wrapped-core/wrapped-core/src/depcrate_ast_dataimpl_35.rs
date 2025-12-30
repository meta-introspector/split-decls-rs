// Generated macro for impl_35 (impl)
macro_rules! Depcrate_ast_dataimpl_35 {
() => {
// Module: crate::ast::data
// Provides: {"impl_35"}
// Dependencies: {}
impl < F : FromField > Fields < F > { pub fn try_from (fields : & syn :: Fields) -> Result < Self > { let mut errors = Error :: accumulator () ; let items = { match & fields { syn :: Fields :: Named (fields) => fields . named . iter () . filter_map (| field | { errors . handle (FromField :: from_field (field) . map_err (| err | { if let Some (ident) = & field . ident { err . at (ident) } else { err } })) }) . collect () , syn :: Fields :: Unnamed (fields) => fields . unnamed . iter () . filter_map (| field | errors . handle (FromField :: from_field (field))) . collect () , syn :: Fields :: Unit => vec ! [] , } } ; errors . finish () ? ; Ok (Self :: new (fields . into () , items) . with_span (fields . span ())) } }
};
}
