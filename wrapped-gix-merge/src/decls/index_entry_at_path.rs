macro_rules! deps {
    () => {
        ConflictIndexEntryPathHint!();
        ConflictIndexEntry!();
    };
}

macro_rules! index_entry_at_path {
    () => {
        deps!();
        fn index_entry_at_path (mode : & gix_object :: tree :: EntryMode , id : & gix_hash :: ObjectId , hint : ConflictIndexEntryPathHint ,) -> Option < ConflictIndexEntry > { Some (ConflictIndexEntry { mode : * mode , id : * id , path_hint : Some (hint) , }) }
    };
}

index_entry_at_path!();