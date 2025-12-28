macro_rules! UnsafeUseReason {
    () => {
        # [derive (Debug , Copy , Clone)] pub (crate) enum UnsafeUseReason { Call , Method , Path , UnionField , Deref , }
    };
}

UnsafeUseReason!()