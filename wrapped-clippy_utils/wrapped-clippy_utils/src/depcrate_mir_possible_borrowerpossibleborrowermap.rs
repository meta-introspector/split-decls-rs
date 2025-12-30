// Generated macro for PossibleBorrowerMap (struct)
macro_rules! Depcrate_mir_possible_borrowerPossibleBorrowerMap {
() => {
// Module: crate::mir::possible_borrower
// Provides: {"PossibleBorrowerMap"}
// Dependencies: {}
# [doc = " Result of `PossibleBorrowerVisitor`."] pub struct PossibleBorrowerMap < 'b , 'tcx > { # [doc = " Mapping `Local -> its possible borrowers`"] pub map : FxHashMap < mir :: Local , DenseBitSet < mir :: Local > > , maybe_live : ResultsCursor < 'b , 'tcx , MaybeStorageLive < 'tcx > > , pub bitset : (DenseBitSet < mir :: Local > , DenseBitSet < mir :: Local >) , }
};
}
