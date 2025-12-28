macro_rules! deps {
    () => {
        SourceRoot!();
    };
}

macro_rules! file_text_durability {
    () => {
        deps!();
        fn file_text_durability (source_root : & SourceRoot) -> Durability { if source_root . is_library { Durability :: HIGH } else { Durability :: LOW } }
    };
}

file_text_durability!()