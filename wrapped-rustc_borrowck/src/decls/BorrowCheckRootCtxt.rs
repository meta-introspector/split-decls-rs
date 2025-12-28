macro_rules! deps {
    () => {
        PropagatedBorrowCheckResults!();
        CollectRegionConstraintsResult!();
        BorrowckConsumer!();
    };
}

macro_rules! BorrowCheckRootCtxt {
    () => {
        deps!();
        # [doc = " The shared context used by both the root as well as all its nested"] # [doc = " items."] pub (super) struct BorrowCheckRootCtxt < 'tcx > { pub tcx : TyCtxt < 'tcx > , root_def_id : LocalDefId , concrete_opaque_types : ConcreteOpaqueTypes < 'tcx > , # [doc = " The region constraints computed by [borrowck_collect_region_constraints]. This uses"] # [doc = " an [FxIndexMap] to guarantee that iterating over it visits nested bodies before"] # [doc = " their parents."] collect_region_constraints_results : FxIndexMap < LocalDefId , CollectRegionConstraintsResult < 'tcx > > , propagated_borrowck_results : FxHashMap < LocalDefId , PropagatedBorrowCheckResults < 'tcx > > , tainted_by_errors : Option < ErrorGuaranteed > , # [doc = " This should be `None` during normal compilation. See [`crate::consumers`] for more"] # [doc = " information on how this is used."] pub consumer : Option < BorrowckConsumer < 'tcx > > , }
    };
}

BorrowCheckRootCtxt!();