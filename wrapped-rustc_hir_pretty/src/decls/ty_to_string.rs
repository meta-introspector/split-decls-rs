macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! ty_to_string {
    () => {
        deps!();
        pub fn ty_to_string (ann : & dyn PpAnn , ty : & hir :: Ty < '_ >) -> String { to_string (ann , | s | s . print_type (ty)) }
    };
}

ty_to_string!();