// Generated macro for required_features (function)
macro_rules! Depcrate_target_specrequired_features {
() => {
// Module: crate::target_spec
// Provides: {"required_features"}
// Dependencies: {}
# [doc = " Fill minimal features needed"] fn required_features (cfg_expr : & CfgExpr , features : & mut Vec < String >) { match cfg_expr { CfgExpr :: Atom (CfgAtom :: KeyValue { key , value }) if * key == sym :: feature => { features . push (value . to_string ()) } CfgExpr :: All (preds) => { preds . iter () . for_each (| cfg | required_features (cfg , features)) ; } CfgExpr :: Any (preds) => { for cfg in preds . iter () { let len_features = features . len () ; required_features (cfg , features) ; if len_features != features . len () { break ; } } } _ => { } } }
};
}
