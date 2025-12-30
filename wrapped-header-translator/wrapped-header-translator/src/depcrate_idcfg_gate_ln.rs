// Generated macro for cfg_gate_ln (function)
macro_rules! Depcrate_idcfg_gate_ln {
() => {
// Module: crate::id
// Provides: {"cfg_gate_ln"}
// Dependencies: {}
# [doc = " Helper to emit a `#[cfg(feature = \"...\")]`-gate based on the required"] # [doc = " items and the implied features."] # [doc = ""] # [doc = " The emission location is also considered an \"implied\" item."] pub fn cfg_gate_ln < 'a , R : AsRef < ItemTree > + 'a , I : AsRef < ItemTree > + 'a > (required : impl IntoIterator < Item = R > + 'a , implied : impl IntoIterator < Item = I > + 'a , config : & 'a Config , emission_location : & 'a Location ,) -> impl fmt :: Display + 'a { let mut feature_names = BTreeSet :: new () ; let mut platform_cfg = PlatformCfg :: from_config (config . library (emission_location)) ; for item in required { let item : & ItemTree = item . as_ref () ; feature_names . extend (item . cfg_features (config , emission_location)) ; item . visit (emission_location , | id , _ | { platform_cfg . dependency (config . library (id)) ; }) ; } for item in implied { let item : & ItemTree = item . as_ref () ; for feature_name in item . cfg_features (config , emission_location) { feature_names . remove (& feature_name) ; } item . visit (emission_location , | id , _ | { platform_cfg . implied (config . library (id)) ; }) ; } for feature_name in ItemTree :: cfg_features_inner (emission_location , config , emission_location) { feature_names . remove (& feature_name) ; } platform_cfg . implied (config . library (emission_location)) ; FormatterFn (move | f | { write ! (f , "{}" , cfg_features_ln (& feature_names)) ? ; if let Some (cfg) = platform_cfg . cfgs () { writeln ! (f , "#[cfg({cfg})]") ? ; } Ok (()) }) }
};
}
