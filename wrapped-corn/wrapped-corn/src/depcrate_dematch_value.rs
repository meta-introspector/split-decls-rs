// Generated macro for match_value (macro)
macro_rules! Depcrate_dematch_value {
() => {
// Module: crate::de
// Provides: {"match_value"}
// Dependencies: {}
macro_rules ! match_value { ($ self : ident , $ name : literal , $ ($ pat : pat => $ expr : expr) +) => { { let value = get_value ! ($ self) ; match value { $ ($ pat => $ expr ,) + _ => err_expected ! ($ name , value) } } } ; }
};
}
