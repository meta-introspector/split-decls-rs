// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cfg_exprimpl_12 {
() => {
// Module: crate::cfg_expr
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (test)] impl arbitrary :: Arbitrary < '_ > for CfgAtom { fn arbitrary (u : & mut arbitrary :: Unstructured < '_ >) -> arbitrary :: Result < Self > { if u . arbitrary () ? { Ok (CfgAtom :: Flag (Symbol :: intern (< _ > :: arbitrary (u) ?))) } else { Ok (CfgAtom :: KeyValue { key : Symbol :: intern (< _ > :: arbitrary (u) ?) , value : Symbol :: intern (< _ > :: arbitrary (u) ?) , }) } } }
};
}
