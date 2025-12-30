// Generated macro for simple_platform_gate (function)
macro_rules! Depcrate_stmtsimple_platform_gate {
() => {
// Module: crate::stmt
// Provides: {"simple_platform_gate"}
// Dependencies: {}
fn simple_platform_gate (data : & LibraryConfig , required_items : impl IntoIterator < Item = ItemTree > , implied_items : impl IntoIterator < Item = ItemTree > , config : & Config ,) -> impl Display { let mut platform_cfg = PlatformCfg :: from_config (data) ; for item in required_items { platform_cfg . dependency (config . library (item . id ())) ; } for item in implied_items { platform_cfg . implied (config . library (item . id ())) ; } FormatterFn (move | f | { if let Some (cfg) = platform_cfg . cfgs () { writeln ! (f , "#[cfg({cfg})]") ? ; } Ok (()) }) }
};
}
