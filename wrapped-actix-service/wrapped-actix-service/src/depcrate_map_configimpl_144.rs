// Generated macro for impl_144 (impl)
macro_rules! Depcrate_map_configimpl_144 {
() => {
// Module: crate::map_config
// Provides: {"impl_144"}
// Dependencies: {}
impl < SF , Req , F , Cfg > Clone for MapConfig < SF , Req , F , Cfg > where SF : Clone , F : Clone , { fn clone (& self) -> Self { Self { factory : self . factory . clone () , cfg_mapper : self . cfg_mapper . clone () , e : PhantomData , } } }
};
}
