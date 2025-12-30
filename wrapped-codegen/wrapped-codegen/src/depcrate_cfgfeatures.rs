// Generated macro for features (function)
macro_rules! Depcrate_cfgfeatures {
() => {
// Module: crate::cfg
// Provides: {"features"}
// Dependencies: {}
pub fn features (features : & Features , doc_cfg : impl Into < DocCfg >) -> TokenStream { let features = & features . any ; let cfg = match features . len () { 0 => None , 1 => Some (quote ! { cfg (feature = # (# features) *) }) , _ => Some (quote ! { cfg (any (# (feature = # features) ,*)) }) , } ; match (cfg , doc_cfg . into ()) { (Some (cfg) , DocCfg :: Ordinary) => quote ! { # [# cfg] # [cfg_attr (docsrs , doc (# cfg))] } , (Some (cfg) , DocCfg :: Override (overriding_cfg)) => quote ! { # [# cfg] # [cfg_attr (docsrs , doc (cfg (feature = # overriding_cfg)))] } , (Some (cfg) , DocCfg :: None) => quote ! { # [# cfg] } , (None , DocCfg :: Override (overriding_cfg)) => quote ! { # [cfg_attr (docsrs , doc (cfg (feature = # overriding_cfg)))] } , (None , DocCfg :: Ordinary | DocCfg :: None) => TokenStream :: new () , } }
};
}
