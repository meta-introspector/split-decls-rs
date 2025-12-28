macro_rules! deps {
    () => {
        CleanupKind!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl CleanupKind { pub (crate) fn funclet_bb (self , for_bb : mir :: BasicBlock) -> Option < mir :: BasicBlock > { match self { CleanupKind :: NotCleanup => None , CleanupKind :: Funclet => Some (for_bb) , CleanupKind :: Internal { funclet } => Some (funclet) , } } }
    };
}

impl_449!()