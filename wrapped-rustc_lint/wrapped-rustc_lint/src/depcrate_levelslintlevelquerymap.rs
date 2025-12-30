// Generated macro for LintLevelQueryMap (struct)
macro_rules! Depcrate_levelsLintLevelQueryMap {
() => {
// Module: crate::levels
// Provides: {"LintLevelQueryMap"}
// Dependencies: {}
struct LintLevelQueryMap < 'tcx > { tcx : TyCtxt < 'tcx > , cur : HirId , specs : ShallowLintLevelMap , # [doc = " Empty hash map to simplify code."] empty : FxIndexMap < LintId , LevelAndSource > , attrs : & 'tcx hir :: AttributeMap < 'tcx > , }
};
}
