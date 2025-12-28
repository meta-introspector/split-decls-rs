macro_rules! deps {
    () => {
        ConflictIndexEntry!();
    };
}

macro_rules! index_entry {
    () => {
        deps!();
        fn index_entry (mode : & gix_object :: tree :: EntryMode , id : & gix_hash :: ObjectId) -> Option < ConflictIndexEntry > { Some (ConflictIndexEntry { mode : * mode , id : * id , path_hint : None , }) }
    };
}

index_entry!();