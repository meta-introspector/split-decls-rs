// Generated macro for impl_322 (impl)
macro_rules! Depcrate_valueimpl_322 {
() => {
// Module: crate::value
// Provides: {"impl_322"}
// Dependencies: {}
impl < T > From < Map < String , T > > for ValueKind where T : Into < Value > , { fn from (values : Map < String , T >) -> Self { let t = values . into_iter () . map (| (k , v) | (k , v . into ())) . collect () ; Self :: Table (t) } }
};
}
