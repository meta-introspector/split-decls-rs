// Generated macro for PossibleOriginVisitor (struct)
macro_rules! Depcrate_mir_possible_originPossibleOriginVisitor {
() => {
// Module: crate::mir::possible_origin
// Provides: {"PossibleOriginVisitor"}
// Dependencies: {}
# [doc = " Collect possible borrowed for every `&mut` local."] # [doc = " For example, `_1 = &mut _2` generate _1: {_2,...}"] # [doc = " Known Problems: not sure all borrowed are tracked"] pub (super) struct PossibleOriginVisitor < 'a , 'tcx > { possible_origin : TransitiveRelation , body : & 'a mir :: Body < 'tcx > , }
};
}
