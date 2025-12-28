macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! is_box {
    () => {
        deps!();
        pub fn is_box (db : & dyn HirDatabase , adt : AdtId) -> bool { let AdtId :: StructId (id) = adt else { return false } ; db . struct_signature (id) . flags . contains (StructFlags :: IS_BOX) }
    };
}

is_box!();