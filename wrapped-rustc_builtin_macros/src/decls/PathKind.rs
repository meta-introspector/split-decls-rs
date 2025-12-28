macro_rules! PathKind {
    () => {
        # [derive (Clone)] pub (crate) enum PathKind { Local , Std , }
    };
}

PathKind!();