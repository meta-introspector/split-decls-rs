macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! qpath_to_string {
    () => {
        deps!();
        pub fn qpath_to_string (ann : & dyn PpAnn , segment : & hir :: QPath < '_ >) -> String { to_string (ann , | s | s . print_qpath (segment , false)) }
    };
}

qpath_to_string!();