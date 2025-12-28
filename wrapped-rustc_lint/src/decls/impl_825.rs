macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_825 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for InvalidAtomicOrdering { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { Self :: check_atomic_load_store (cx , expr) ; Self :: check_memory_fence (cx , expr) ; Self :: check_atomic_compare_exchange (cx , expr) ; } }
    };
}

impl_825!();