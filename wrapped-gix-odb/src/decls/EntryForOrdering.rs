macro_rules! EntryForOrdering {
    () => {
        struct EntryForOrdering { pack_offset : u64 , entry_index : u32 , pack_index : u16 , }
    };
}

EntryForOrdering!()