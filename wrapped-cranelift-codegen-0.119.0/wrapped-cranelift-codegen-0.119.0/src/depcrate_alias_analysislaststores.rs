// Generated macro for LastStores (struct)
macro_rules! Depcrate_alias_analysisLastStores {
() => {
// Module: crate::alias_analysis
// Provides: {"LastStores"}
// Dependencies: {}
# [doc = " For a given program point, the vector of last-store instruction"] # [doc = " indices for each disjoint category of abstract state."] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub struct LastStores { heap : PackedOption < Inst > , table : PackedOption < Inst > , vmctx : PackedOption < Inst > , other : PackedOption < Inst > , }
};
}
