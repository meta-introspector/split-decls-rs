macro_rules! LintLevelQueryMap {
    () => {
        struct LintLevelQueryMap < 'tcx > { tcx : TyCtxt < 'tcx > , cur : HirId , specs : ShallowLintLevelMap , # [doc = " Empty hash map to simplify code."] empty : FxIndexMap < LintId , LevelAndSource > , attrs : & 'tcx hir :: AttributeMap < 'tcx > , }
    };
}

LintLevelQueryMap!()