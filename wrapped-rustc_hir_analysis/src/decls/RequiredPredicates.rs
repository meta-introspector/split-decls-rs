macro_rules! RequiredPredicates {
    () => {
        # [doc = " Tracks the `T: 'a` or `'a: 'a` predicates that we have inferred"] # [doc = " must be added to the struct header."] pub (crate) type RequiredPredicates < 'tcx > = FxIndexMap < ty :: ArgOutlivesPredicate < 'tcx > , Span > ;
    };
}

RequiredPredicates!()