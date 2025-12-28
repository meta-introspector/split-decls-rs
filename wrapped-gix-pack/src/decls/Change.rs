macro_rules! Change {
    () => {
        # [derive (Debug)] struct Change { # [doc = " The original pack offset as mentioned in the entry we saw. This is used to find this as base object if deltas refer to it by"] # [doc = " old offset."] pack_offset : u64 , # [doc = " The new pack offset that is the shifted location of the pack entry in the pack."] shifted_pack_offset : u64 , # [doc = " The size change of the entry header, negative values denote shrinking, positive denote growing."] size_change_in_bytes : i64 , # [doc = " The object id of the entry responsible for the change, or null if it's an entry just for tracking an insertion."] oid : ObjectId , }
    };
}

Change!()