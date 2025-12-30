// Generated macro for impl_149 (impl)
macro_rules! Depcrate_map_configimpl_149 {
() => {
// Module: crate::map_config
// Provides: {"impl_149"}
// Dependencies: {}
impl < SF , Cfg , Req > ServiceFactory < Req > for UnitConfig < SF , Cfg , Req > where SF : ServiceFactory < Req , Config = () > , { type Response = SF :: Response ; type Error = SF :: Error ; type Config = Cfg ; type Service = SF :: Service ; type InitError = SF :: InitError ; type Future = SF :: Future ; fn new_service (& self , _ : Cfg) -> Self :: Future { self . factory . new_service (()) } }
};
}
