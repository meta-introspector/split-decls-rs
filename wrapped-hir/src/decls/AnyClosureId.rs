macro_rules! AnyClosureId {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] enum AnyClosureId { ClosureId (InternedClosureId) , CoroutineClosureId (InternedCoroutineId) , }
    };
}

AnyClosureId!();