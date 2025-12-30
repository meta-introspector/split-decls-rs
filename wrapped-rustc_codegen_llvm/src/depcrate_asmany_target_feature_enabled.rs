// Generated macro for any_target_feature_enabled (function)
macro_rules! Depcrate_asmany_target_feature_enabled {
() => {
// Module: crate::asm
// Provides: {"any_target_feature_enabled"}
// Dependencies: {}
fn any_target_feature_enabled (cx : & CodegenCx < '_ , '_ > , instance : Instance < '_ > , features : & [Symbol] ,) -> bool { let enabled = cx . tcx . asm_target_features (instance . def_id ()) ; features . iter () . any (| feat | enabled . contains (feat)) }
};
}
