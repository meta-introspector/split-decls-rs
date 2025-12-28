macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! pat_to_string {
    () => {
        deps!();
        pub fn pat_to_string (ann : & dyn PpAnn , pat : & hir :: Pat < '_ >) -> String { to_string (ann , | s | s . print_pat (pat)) }
    };
}

pat_to_string!()