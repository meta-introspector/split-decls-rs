macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! item_to_string {
    () => {
        deps!();
        pub fn item_to_string (ann : & dyn PpAnn , pat : & hir :: Item < '_ >) -> String { to_string (ann , | s | s . print_item (pat)) }
    };
}

item_to_string!();