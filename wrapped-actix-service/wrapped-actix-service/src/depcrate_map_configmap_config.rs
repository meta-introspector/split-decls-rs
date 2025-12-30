// Generated macro for map_config (function)
macro_rules! Depcrate_map_configmap_config {
() => {
// Module: crate::map_config
// Provides: {"map_config"}
// Dependencies: {}
# [doc = " Adapt external config argument to a config for provided service factory"] # [doc = ""] # [doc = " Note that this function consumes the receiving service factory and returns"] # [doc = " a wrapped version of it."] pub fn map_config < I , SF , Req , F , Cfg > (factory : I , f : F) -> MapConfig < SF , Req , F , Cfg > where I : IntoServiceFactory < SF , Req > , SF : ServiceFactory < Req > , F : Fn (Cfg) -> SF :: Config , { MapConfig :: new (factory . into_factory () , f) }
};
}
