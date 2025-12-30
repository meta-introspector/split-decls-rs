// Generated macro for impl_75 (impl)
macro_rules! Depcrate_leavesimpl_75 {
() => {
// Module: crate::leaves
// Provides: {"impl_75"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > Property < M , D > where D :: Property : arg :: Append + Clone { # [doc = " Adds a \"standard\" get handler."] pub fn default_get (mut self) -> Self { let g = | i : & mut arg :: IterAppend , p : & PropInfo < M , D > | { i . append (p . prop . get_data ()) ; Ok (()) } ; self . get_cb = Some (DebugGetProp (M :: make_getprop (g))) ; self } }
};
}
