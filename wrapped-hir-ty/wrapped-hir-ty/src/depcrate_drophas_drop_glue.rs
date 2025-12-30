// Generated macro for has_drop_glue (function)
macro_rules! Depcrate_drophas_drop_glue {
() => {
// Module: crate::drop
// Provides: {"has_drop_glue"}
// Dependencies: {}
pub fn has_drop_glue < 'db > (infcx : & InferCtxt < 'db > , ty : Ty < 'db > , env : Arc < TraitEnvironment < 'db > > ,) -> DropGlue { has_drop_glue_impl (infcx , ty , env , & mut FxHashSet :: default ()) }
};
}
