// Generated macro for impl_215 (impl)
macro_rules! Depcrate_typesimpl_215 {
() => {
// Module: crate::types
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'source , T > From < Option < T > > for FluentValue < 'source > where T : Into < FluentValue < 'source > > , { fn from (v : Option < T >) -> Self { match v { Some (v) => v . into () , None => FluentValue :: None , } } }
};
}
