// Generated macro for unit_config (function)
macro_rules! Depcrate_map_configunit_config {
() => {
// Module: crate::map_config
// Provides: {"unit_config"}
// Dependencies: {}
# [doc = " Replace config with unit."] pub fn unit_config < I , SF , Cfg , Req > (factory : I) -> UnitConfig < SF , Cfg , Req > where I : IntoServiceFactory < SF , Req > , SF : ServiceFactory < Req , Config = () > , { UnitConfig :: new (factory . into_factory ()) }
};
}
