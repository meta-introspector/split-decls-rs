macro_rules! deps {
    () => {
        SourceRoot!();
    };
}

macro_rules! source_root_durability {
    () => {
        deps!();
        fn source_root_durability (source_root : & SourceRoot) -> Durability { if source_root . is_library { Durability :: MEDIUM } else { Durability :: LOW } }
    };
}

source_root_durability!();