// Generated macro for PossibleBorrowerVisitor (struct)
macro_rules! Depcrate_mir_possible_borrowerPossibleBorrowerVisitor {
() => {
// Module: crate::mir::possible_borrower
// Provides: {"PossibleBorrowerVisitor"}
// Dependencies: {}
# [doc = " Collects the possible borrowers of each local."] # [doc = " For example, `b = &a; c = &a;` will make `b` and (transitively) `c`"] # [doc = " possible borrowers of `a`."] struct PossibleBorrowerVisitor < 'a , 'b , 'tcx > { possible_borrower : TransitiveRelation , body : & 'b mir :: Body < 'tcx > , cx : & 'a LateContext < 'tcx > , possible_origin : FxHashMap < mir :: Local , DenseBitSet < mir :: Local > > , }
};
}
