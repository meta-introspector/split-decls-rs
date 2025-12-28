macro_rules! deps {
    () => {
        InferredIndex!();
        VarianceTermPtr!();
    };
}

macro_rules! TermsContext {
    () => {
        deps!();
        # [doc = " The first pass over the crate simply builds up the set of inferreds."] pub (crate) struct TermsContext < 'a , 'tcx > { pub tcx : TyCtxt < 'tcx > , pub arena : & 'a DroplessArena , # [doc = " For marker types, `UnsafeCell`, and other lang items where"] # [doc = " variance is hardcoded, records the item-id and the hardcoded"] # [doc = " variance."] pub lang_items : Vec < (LocalDefId , Vec < ty :: Variance >) > , # [doc = " Maps from the node id of an item to the first inferred index"] # [doc = " used for its type & region parameters."] pub inferred_starts : LocalDefIdMap < InferredIndex > , # [doc = " Maps from an InferredIndex to the term for that variable."] pub inferred_terms : Vec < VarianceTermPtr < 'a > > , }
    };
}

TermsContext!();