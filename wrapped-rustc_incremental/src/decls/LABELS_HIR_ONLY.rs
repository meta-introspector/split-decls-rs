macro_rules! LABELS_HIR_ONLY {
    () => {
        # [doc = " For generic cases like inline-assembly, modules, etc."] const LABELS_HIR_ONLY : & [& [& str]] = & [BASE_HIR] ;
    };
}

LABELS_HIR_ONLY!();