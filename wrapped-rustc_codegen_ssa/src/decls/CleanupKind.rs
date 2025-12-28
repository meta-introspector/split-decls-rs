macro_rules! CleanupKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum CleanupKind { NotCleanup , Funclet , Internal { funclet : mir :: BasicBlock } , }
    };
}

CleanupKind!();