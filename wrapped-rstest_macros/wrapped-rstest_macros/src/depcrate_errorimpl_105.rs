// Generated macro for impl_105 (impl)
macro_rules! Depcrate_errorimpl_105 {
() => {
// Module: crate::error
// Provides: {"impl_105"}
// Dependencies: {}
impl RenderType for syn :: Pat { fn render_type (& self) -> String { match self { syn :: Pat :: Ident (ref i) => i . ident . to_string () , other => format ! ("{other:?}") , } } }
};
}
