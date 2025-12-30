// Generated macro for impl_145 (impl)
macro_rules! Depcrate_map_configimpl_145 {
() => {
// Module: crate::map_config
// Provides: {"impl_145"}
// Dependencies: {}
impl < SF , Req , F , Cfg > ServiceFactory < Req > for MapConfig < SF , Req , F , Cfg > where SF : ServiceFactory < Req > , F : Fn (Cfg) -> SF :: Config , { type Response = SF :: Response ; type Error = SF :: Error ; type Config = Cfg ; type Service = SF :: Service ; type InitError = SF :: InitError ; type Future = SF :: Future ; fn new_service (& self , cfg : Self :: Config) -> Self :: Future { let mapped_cfg = (self . cfg_mapper) (cfg) ; self . factory . new_service (mapped_cfg) } }
};
}
