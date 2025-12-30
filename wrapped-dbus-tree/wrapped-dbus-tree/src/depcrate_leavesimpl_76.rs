// Generated macro for impl_76 (impl)
macro_rules! Depcrate_leavesimpl_76 {
() => {
// Module: crate::leaves
// Provides: {"impl_76"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > Property < M , D > where D :: Property : arg :: RefArg { # [doc = " Adds a \"standard\" get handler (for RefArgs)."] pub fn default_get_refarg (mut self) -> Self { let g = | i : & mut arg :: IterAppend , p : & PropInfo < M , D > | { arg :: RefArg :: append (p . prop . get_data () , i) ; Ok (()) } ; self . get_cb = Some (DebugGetProp (M :: make_getprop (g))) ; self } }
};
}
