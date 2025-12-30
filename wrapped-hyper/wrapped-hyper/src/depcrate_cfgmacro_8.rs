// Generated macro for macro_8 (macro)
macro_rules! Depcrate_cfgmacro_8 {
() => {
// Module: crate::cfg
// Provides: {"macro_8"}
// Dependencies: {}
cfg_proto ! { macro_rules ! cfg_client { ($ ($ item : item) *) => { cfg_feature ! { #! [feature = "client"] $ ($ item) * } } } macro_rules ! cfg_server { ($ ($ item : item) *) => { cfg_feature ! { #! [feature = "server"] $ ($ item) * } } } }
};
}
